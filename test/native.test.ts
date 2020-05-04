const { ReportAuthorizationKey, MemoType } = require("../src/index").native;

describe("TCN native module", () => {
  // Below is taken directly from the Rust example at https://docs.rs/tcn/0.4.1/tcn/
  // with minimal modifications to make it work in JS
  it("should support the example usage from the TCN docs", () => {
    // Generate a report authorization key.  This key represents the capability
    // to publish a report about a collection of derived temporary contact numbers.
    let rak = new ReportAuthorizationKey();

    // Use the temporary contact key ratchet mechanism to compute a list
    // of temporary contact numbers.
    let tck = rak.initial_temporary_contact_key(); // tck <- tck_1
    let tcns = Array.from({ length: 100 }, () => {
      let tcn = tck.temporary_contact_number();
      tck = tck.ratchet();
      return tcn;
    });
    expect(tcns).toHaveLength(100);

    // Prepare a report about a subset of the temporary contact numbers.
    let signed_report = rak.create_report(
      MemoType.CoEpiV1, // The memo type
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
    expect(recomputed_tcns).toEqual(tcns.slice(20 - 1, 90 - 1));
  });
});
