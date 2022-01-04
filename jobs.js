const fs = require("fs");
const logat = require("logat");
const utils = require("./utils");
const crypto = require("crypto");
const readline = require("readline");

var getHashedPathName = function(input) {
	return crypto.createHash('md5').update(input).digest('hex');
};

var sanitizeLine = function(line, allRules) {
	// Here try to understand what kind of rule it is.
	// If it is a regular expression, handle it like a regular expression update.
	return line;
};

var runSanitizerForInputAndOutputPaths = function(inputPath, outputPath, allRules, masterConfig) {
	logat.debug("Starting for path", inputPath);

	var hashOfInputFilePath = getHashedPathName(inputPath); // creating hash for making it unique
	var fileThatContainsLinesProcessed = masterConfig.tmpDirectory + "/" + hashOfInputFilePath;
	utils.createFileIfNotExists(fileThatContainsLinesProcessed, "0");
	// logat.debug("utils.getFileContentUTF8(fileThatContainsLinesProcessed)", utils.getFileContentUTF8(fileThatContainsLinesProcessed));
	var currentLineNumber = parseInt(utils.getFileContentUTF8(fileThatContainsLinesProcessed));

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

	// var lineNumberFileStream = fs.createWriteStream(fileThatContainsLinesProcessed, {});

	var inputFileLineNumber = 0;

	rl.on('line', (line) => {
			inputFileLineNumber++;
			if (inputFileLineNumber > currentLineNumber) {
				currentLineNumber++;
				outputStream.write(sanitizeLine(line, allRules) + "\n");
				// lineNumberFileStream.write(currentLineNumber.toString());
				fs.writeFileSync(fileThatContainsLinesProcessed, currentLineNumber.toString());
			}

		})
		.on('close', () => {
			outputStream.end();
			// lineNumberFileStream.end();
			logat.debug("Completed whatever was there for this file", inputPath);
		});
};

var initiateThisJob = function(thisJob, allRules, masterConfig) {

	runSanitizerForInputAndOutputPaths(thisJob.files.input, thisJob.files.output, allRules, masterConfig);

};

var initiateTheJobs = function(allRules, allJobs, masterConfig) {
	logat.debug(allRules, allJobs, masterConfig);

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