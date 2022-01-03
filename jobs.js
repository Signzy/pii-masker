const fs = require("fs");
const logat = require("logat");
const utils = require("./utils");
const crypto = require("crypto");
const readline = require("readline");

var getHashedPathName = function(input) {
	return crypto.createHash('sha1').update(input).digest('base64');
};

var sanitizeLine = function(line, allRules) {
	// Here try to understand what kind of rule it is.
	// If it is a regular expression, handle it like a regular expression update.
	return line;
};

var runSanitizerForInputAndOutputPaths = function(inputPath, startLine, outputPath, allRules) {

	utils.createFileIfNotExists(outputPath);

	const inputFileStream = fs.createReadStream(inputPath);

	const rl = readline.createInterface({
		input: inputFileStream,
		crlfDelay: Infinity
	});

	var outputStream = fs.createWriteStream(outputPath, {
		flags: 'a' // 'a' means appending (old data will be preserved)
	})

	rl.on('line', (line) => {
			outputStream.write(sanitizeLine(line, allRules) + "\n");
		})
		.on('close', () => {
			fs.writeFileSync(masterConfig.tmpDirectory);
		});
};

var initiateThisJob = function(thisJob, allRules, masterConfig) {
	var hashedFileName = getHashedPathName(thisJob.files.input);
	var tmpFileName = masterConfig.tmpDirectory + "/" + hashedFileName;
	utils.createFileIfNotExists(tmpFileName, "0");

	var startLine = parseInt(utils.getFileContentUTF8(tmpFileName));

	runSanitizerForInputAndOutputPaths(thisJob.files.input, startLine, thisJob.files.output, allRules, masterConfig);

};

var initiateTheJobs = function(allRules, allJobs, masterConfig) {
	logat.debug(allRules, allJobs, masterConfig);

	// Here I will simply make fire and forget call to the node function
	// That will check if process is already running or not
	// And if not running for that file, it will initiate it.

	for (var i = 0; i < allJobs.length; i++) {
		initiateThisJob(allJobs[i], allRules, masterConfig)
	}

};

module.exports = {};
module.exports.run = initiateTheJobs;