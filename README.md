# Purpose of the script

Write a streamer that can take a log file and stream the sanitized log lines from the production data.

## Rough notes.

The purpose of this system is to clean up the logs and other entries
- First target is to clean up the log files (or any other files for that matter.)
- ther locations of the PII data will come in later on.

Create a default list of type of data. This types of data will be maintained by the repository itself
There should be user configurable list of types of data also that the system can take as an input.

The user can enter their own rules additionally.

### Job definition
This system should be able to register the jobs. One job is one input file to be taken for sanitization.

The user can configure multiple jobs into this system.

The job definition can look like the below.

```
{
    "jobName": "",
    files: {
        "input": "<absolute path to the input file>",
        "output": "<absolute path to where the output file>",
    }
}
```

System would create a read stream on the input file and the write (append) stream on the output file.

The input would be read line by line and parsed at a target speed of 200 lines per second.

### Rules

A rule is configured in this manner.

```
[{
    "type": "regex",
    "definition": "[A-Z]{5}[0-9]{4}[A-Z]{1}",
    "nameForThisRule": "PAN rule"
}]
```

Note the redaction Rule parameter, basically it will redact all characters except the first 3 and last 2.

### Configurable parameters
- How many characters to leave unmasked (default 4)
- Speed per second, default is 200
- User's masking rules
- jobRefreshInterval - Refresh Cycle time - defines at what interval should the jobs be refreshed, default is 60 seconds.

### Target speed
Service should be able to run very fast, so that the number of lines that it can process is matching with the speed at which a logging system may generate the logs. - Target is 200 lines per second in a 8 core machine.


### Expected details on the output lines

Should have a marker, on the entry time of the log and the entry time in the filtered stream.

#### Other points to be evaluated

Should we provide a method to enable/ disable restful APIs for registering additional details.

The system should be able to keep somewhere till what line number has the system already processed, so upon the re-registration of the system, we can pick from where we left and not the starting.

The reason for creating this as a streamer over files and not as an extension to logger framework is so that it can be reused for other log systems like Nginx, where the request body could be getting logged in.

# Notes

- In case of conflicting rules the first one to appear shall be applied. In case of conflict between default rules and the user rules, the default rules take precedence and hence they'll override anything coming later.

Still thought to be given to the below points;
Applies to basically filters the rules to be applied, for example the rule type of Nginx and NodeJS logs may be different. "appliesTo": [""] can be used, but we need to think if this is of much value add and adds to the system's speed ?

