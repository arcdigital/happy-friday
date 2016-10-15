const spawnSync = require('child_process').spawnSync

exports.handler = function (event, context, callback) {
    process.env.PATH = process.env.PATH + ':' + process.env.LAMBDA_TASK_ROOT
    var result = spawnSync('./happy-friday')
    callback(null, result.output.join("\n"))
}
