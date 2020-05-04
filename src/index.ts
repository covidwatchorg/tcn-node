export const native = require("../native");

export const tcnExample = () => {
  const assert = require("assert");

  // Generate a report authorization key.  This key represents the capability
  // to publish a report about a collection of derived temporary contact numbers.
  let rak = new native.ReportAuthorizationKey();

  // Use the temporary contact key ratchet mechanism to compute a list
  // of temporary contact numbers.
  let tck = rak.initial_temporary_contact_key(); // tck <- tck_1
  let tcns = [];
  for (let i = 0; i < 100; i++) {
    tcns.push(tck.temporary_contact_number());
    tck = tck.ratchet();
  }

  // Prepare a report about a subset of the temporary contact numbers.
  let signed_report = rak.create_report(
    native.MemoType.CoEpiV1, // The memo type
    Buffer.from("symptom data"), // The memo data
    20, // Index of the first TCN to disclose
    90 // Index of the last TCN to check
  );

  // Verify the source integrity of the report...
  let report = signed_report.verify();
  // ...allowing the disclosed TCNs to be recomputed.
  let recomputed_tcns = report.temporary_contact_numbers();

  // Check that the recomputed TCNs match the originals.
  // The slice is offset by 1 because tcn_0 is not included.
  assert.deepEqual(recomputed_tcns, tcns.slice(20 - 1, 90 - 1));

  // Read the memo data from the report
  return Buffer.from(report.toObject().memo_data).toString();
};

// also include the old function name so as to not break users of older versions of this library
export const tcn_example = tcnExample;

export const signedReportExample = () =>
  new native.ReportAuthorizationKey()
    .create_report(native.MemoType.CoEpiV1, Buffer.from("symptom data"), 20, 90)
    .toObject();

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

  let signedReport = new native.SignedReport(r);
  try {
    signedReport.verify();
    return true;
  } catch (error) {
    return false;
  }
};
