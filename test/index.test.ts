import { tcnExample, signedReportExample, validateReport } from "../src/index";

describe("tcnExample", () => {
  it("should return 'symptom data'", () => {
    expect(tcnExample()).toBe("symptom data");
  });
});

describe("signedReportExample", () => {
  let signedReport = signedReportExample();
  it("should include a report with the expected values", () => {
    expect(signedReport.report.j_1).toBe(20);
    expect(signedReport.report.j_2).toBe(90);
    expect(signedReport.report.memo_type).toBe("CoEpiV1");
  });
  it("should have a valid signature", () => {
    expect(validateReport(signedReport)).toBe(true);
  });
});

const exampleSignedReport = {
  report: {
    rvk: [
      205,
      234,
      147,
      231,
      210,
      96,
      99,
      128,
      241,
      255,
      168,
      61,
      243,
      222,
      144,
      41,
      194,
      92,
      112,
      118,
      140,
      98,
      90,
      38,
      156,
      32,
      216,
      117,
      171,
      14,
      206,
      117,
    ],
    tck_bytes: [
      5,
      44,
      47,
      43,
      14,
      249,
      162,
      165,
      139,
      157,
      225,
      217,
      38,
      77,
      151,
      140,
      247,
      198,
      138,
      23,
      208,
      188,
      229,
      189,
      20,
      101,
      126,
      83,
      216,
      18,
      194,
      19,
    ],
    j_1: 20,
    j_2: 90,
    memo_type: "CoEpiV1",
    memo_data: [115, 121, 109, 112, 116, 111, 109, 32, 100, 97, 116, 97],
  },
  sig: {
    R_bytes: [
      171,
      0,
      174,
      55,
      138,
      201,
      100,
      209,
      69,
      98,
      176,
      85,
      27,
      240,
      129,
      22,
      204,
      209,
      89,
      245,
      9,
      31,
      170,
      4,
      1,
      69,
      243,
      251,
      36,
      31,
      249,
      192,
    ],
    s_bytes: [
      250,
      99,
      139,
      105,
      167,
      126,
      136,
      208,
      253,
      158,
      225,
      46,
      81,
      179,
      50,
      90,
      113,
      63,
      235,
      163,
      172,
      193,
      251,
      86,
      76,
      118,
      188,
      170,
      16,
      252,
      132,
      8,
    ],
  },
};

describe("validateReport", () => {
  it("should return true for the example signed report in the README", () => {
    expect(validateReport(exampleSignedReport)).toBe(true);
  });
  it("should return false if the report's memo data is modified", () => {
    exampleSignedReport.report.memo_data.push(0);
    expect(validateReport(exampleSignedReport)).toBe(false);
  });
});
