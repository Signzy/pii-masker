const fs = require('fs');

var createFileIfNotExists = function(thisFilePath, content) {
	if (!fs.existsSync(thisFilePath)) {
		fs.writeFileSync(thisFilePath, content ? content : "", {
			flag: 'wx'
		});
	}
	return true;
};

var getFileContentUTF8 = function(filePath) {
	return fs.readFileSync(filePath, 'utf-8');
};

module.exports = {};
module.exports.createFileIfNotExists = createFileIfNotExists;
module.exports.getFileContentUTF8 = getFileContentUTF8;