const native = require("../native");

export const tcnExample = native.tcnExample;
// also include the old function name so as to not break users of older versions of this library
export const tcn_example = tcnExample;

export const signedReportExample = native.signedReportExample;

export const validateReport = (r: any) => {
  // fix for neon-serde deserialization issue - rvk must be an array in an array
  if (
    r &&
    r.report &&
    r.report.rvk &&
    Array.isArray(r.report.rvk) &&
    !Array.isArray(r.report.rvk[0])
  ) {
    r.report.rvk = [r.report.rvk];
  }
  return native.validateReport(r);
};
