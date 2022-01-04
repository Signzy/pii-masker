const fs = require("fs");
const logat = require("logat");
const utils = require("./utils");
const crypto = require("crypto");
const readline = require("readline");

var getHashedPathName = function(input) {
	return crypto.createHash('md5').update(input).digest('hex');
};

var sanitizeLine = function(line, allRules) {
	for (var i = 0; i < allRules.length; i++) {
		var thisRule = allRules[i];
		var thisRuleType = thisRule.type;
		if (thisRule.type == "regex") {
			line = line.split(new RegExp(thisRule.definition)).join(`...XXX ${thisRule.name} XXX...`)
		} else {
			// Do nothing
		}
	}
	return line;
};

var runSanitizerForInputAndOutputPaths = function(inputPath, outputPath, allRules, masterConfig) {

	var hashOfInputFilePath = getHashedPathName(inputPath); // creating hash for making it unique
	var fileThatContainsRunningStatus = masterConfig.tmpDirectory + "/" + hashOfInputFilePath + ".running";
	var fileThatContainsLinesProcessed = masterConfig.tmpDirectory + "/" + hashOfInputFilePath + ".linenumber";

	utils.createFileIfNotExists(fileThatContainsRunningStatus, "0");
	utils.createFileIfNotExists(fileThatContainsLinesProcessed, "0");
	var currentLineNumber = parseInt(utils.getFileContentUTF8(fileThatContainsLinesProcessed));
	var originalStartingLineNumber = currentLineNumber;

	logat.debug(`currentLineNumber = ${currentLineNumber}, originalStartingLineNumber = ${originalStartingLineNumber}`)
	// logat.debug(utils.getFileContentUTF8(fileThatContainsLinesProcessed));
	var isItRunning = parseInt(utils.getFileContentUTF8(fileThatContainsRunningStatus));

	logat.debug("is it already running", isItRunning);
	logat.debug("Starting for path", inputPath);
	logat.debug("Startin from line number", currentLineNumber, "stored at", fileThatContainsLinesProcessed);

	var startTime = new Date();

	if (isItRunning == 0) {
		fs.writeFileSync(fileThatContainsRunningStatus, "1");

		logat.debug("Starting from line number", currentLineNumber, fileThatContainsLinesProcessed);

		utils.createFileIfNotExists(outputPath);

		const inputFileStream = fs.createReadStream(inputPath);

		const rl = readline.createInterface({
			input: inputFileStream,
			crlfDelay: Infinity
		});

		var outputStream = fs.createWriteStream(outputPath, {
			flags: 'a' // 'a' means appending (old data will be preserved)
		});

		var inputFileLineNumber = 0;

		rl
			.on('line', (line) => {
				inputFileLineNumber++;
				// logat.debug(line)
				// logat.debug(`inputFileLineNumber = ${inputFileLineNumber}, currentLineNumber = ${currentLineNumber}`)
				if (inputFileLineNumber > currentLineNumber) {
					// logat.debug(`inputFileLineNumber = ${inputFileLineNumber}, originalStartingLineNumber = ${originalStartingLineNumber}, masterConfig.maxLinesInOneGo = ${masterConfig.maxLinesInOneGo} --`)
					if (inputFileLineNumber > originalStartingLineNumber + masterConfig.maxLinesInOneGo) {
						rl.close();
					} else {
						currentLineNumber++;
						outputStream.write(sanitizeLine(line, allRules) + "\n");
						fs.writeFileSync(fileThatContainsLinesProcessed, currentLineNumber.toString());
					}
				}
			})
			.on('close', () => {
				var endTime = new Date();
				var timeDifference = endTime - startTime;

				logat.debug(`Finished processing in ${timeDifference} milli-seconds at speed of ${Math.floor((inputFileLineNumber - originalStartingLineNumber)*1000/timeDifference)} lines per second`);
				outputStream.end();

				fs.writeFileSync(fileThatContainsRunningStatus, "0");

				logat.debug("Completed whatever was there for this file", inputPath);
			});
	}
};

var initiateThisJob = function(thisJob, allRules, masterConfig) {

	runSanitizerForInputAndOutputPaths(thisJob.files.input, thisJob.files.output, allRules, masterConfig);

};

var initiateTheJobs = function(allRules, allJobs, masterConfig) {
	logat.debug("--------------------------------------------------");
	// logat.debug(allRules, allJobs, masterConfig);

	// ///////////////////////////////////////////////////////
	// Below block is only for sanity purpose
	var allowedRuleTypes = ["regex"];
	for (var i = 0; i < allRules.length; i++) {
		var thisRuleType = allRules[i].type;
		if (allowedRuleTypes.indexOf(thisRuleType) == -1) {
			logat.error(`This kind of rule is not allowed yet - ${thisRuleType}. Allowed types are ${allowedRuleTypes.join(", ")}. All occurrences of this rule in the system will be simply ignored. Refer documentation for more details.`)
		}
	}
	// ///////////////////////////////////////////////////////

	// Here I will simply make fire and forget call to the node function
	// That will check if process is already running or not
	// And if not running for that file, it will initiate it.

	for (var i = 0; i < allJobs.length; i++) {
		initiateThisJob(allJobs[i], allRules, masterConfig)
	}

	return true;
};

module.exports = {};
module.exports.run = initiateTheJobs;