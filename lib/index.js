const native = require('../native');

module.exports = {
  tcnExample: native.tcnExample,
  // also include the old function name so as to not break users of older versions of this library
  tcn_example: native.tcnExample,
  signedReportExample: native.signedReportExample,
  validateReport: r => {
    // fix for neon-serde deserialization issue - rvk must be an array in an array
    if (r && r.report && r.report.rvk && !(r.report.rvk[0] instanceof Array)) {
      r.report.rvk = [r.report.rvk];
    }
    return native.validateReport(r);
  }
}
