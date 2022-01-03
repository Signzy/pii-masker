// Pick the service configurations
// Pick the streams (jobs) from the streams json
// Pick the rules (default) and the user rules, and the concatenate them.

// Register the streams, and start listeners.

// Every listener should basically pick from where it left last time and start from there.
// We should detect if that a block of that file is already taken into the stream, if yes, then wait for another interval.
// Interval can be 5 seconds.

const fs = require("fs");
const logat = require("logat");
const readLine = require("readline");
const crypto = require("crypto");

const jobsRunner = require('./jobs.js')
const utils = require('./utils.js')

const masterConfig = JSON.parse(utils.getFileContentUTF8("./config/master.json"));
const allJobs = JSON.parse(utils.getFileContentUTF8("./config/jobs.json")).jobs;

const userRules = JSON.parse(utils.getFileContentUTF8("./config/rules-user.json"));
const defaultRules = JSON.parse(utils.getFileContentUTF8("./config/rules-default.json"));
logat.debug(userRules, defaultRules);
const allRules = userRules.concat(defaultRules);

// Running the job at the defined interval
setInterval(jobsRunner.run(allRules, allJobs, masterConfig), masterConfig.beatInterval);