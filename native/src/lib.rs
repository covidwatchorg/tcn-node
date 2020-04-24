#[macro_use]
extern crate neon_serde;

use neon::prelude::*;
use std::str;
use tcn::*;

export! {

    fn tcnExample() -> String {
        // Generate a report authorization key.  This key represents the capability
        // to publish a report about a collection of derived temporary contact numbers.
        let rak = ReportAuthorizationKey::new(rand::thread_rng());

        // Use the temporary contact key ratchet mechanism to compute a list
        // of temporary contact numbers.
        let mut tck = rak.initial_temporary_contact_key(); // tck <- tck_1
        let mut tcns = Vec::new();
        for _ in 0..100 {
            tcns.push(tck.temporary_contact_number());
            tck = tck.ratchet().unwrap();
        }

        // Prepare a report about a subset of the temporary contact numbers.
        let signed_report = rak
            .create_report(
                MemoType::CoEpiV1,        // The memo type
                b"symptom data".to_vec(), // The memo data
                20,                       // Index of the first TCN to disclose
                90,                       // Index of the last TCN to check
            )
            .expect("Report creation can only fail if the memo data is too long");

        // Verify the source integrity of the report...
        let report = signed_report
            .verify()
            .expect("Valid reports should verify correctly");

        // ...allowing the disclosed TCNs to be recomputed.
        let recomputed_tcns = report.temporary_contact_numbers().collect::<Vec<_>>();

        // Check that the recomputed TCNs match the originals.
        // The slice is offset by 1 because tcn_0 is not included.
        assert_eq!(&recomputed_tcns[..], &tcns[20 - 1..90 - 1]);

        // Read the memo data from the report
        let memo_data =
            str::from_utf8(report.memo_data()).expect("Could not convert memo bytes to string");

        String::from(memo_data)
    }

    fn signedReportExample() -> SignedReport {
        let rak = ReportAuthorizationKey::new(rand::thread_rng());

        let signed_report = rak
            .create_report(
                MemoType::CoEpiV1,        // The memo type
                b"symptom data".to_vec(), // The memo data
                20,                       // Index of the first TCN to disclose
                90,                       // Index of the last TCN to check
            )
            .expect("Report creation can only fail if the memo data is too long");

        signed_report
    }

    fn validateReport(signed_report: SignedReport) -> bool {
        signed_report.verify().is_ok()
    }
}
