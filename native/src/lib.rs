use neon::prelude::*;
use tcn::*;

fn to_buffer<'a, C: Context<'a>>(cx: &mut C, bytes: &[u8]) -> JsResult<'a, JsBuffer> {
    let buf = cx.buffer(bytes.len() as u32)?;
    buf.borrow(&cx.lock()).as_mut_slice().copy_from_slice(bytes);
    Ok(buf)
}

declare_types! {

    pub class JsReportAuthorizationKey for ReportAuthorizationKey {
        init(_cx) {
            Ok(ReportAuthorizationKey::new(rand::thread_rng()))
        }

        method initial_temporary_contact_key(mut cx) {
            let this = cx.this();
            Ok(JsTemporaryContactKey::new(&mut cx, vec![this])?.upcast())
        }

        method create_report(mut cx) {
            let this = cx.this().upcast();
            let memo_type = cx.argument::<JsValue>(0)?;
            let memo_data = cx.argument::<JsValue>(1)?;
            let start_index = cx.argument::<JsValue>(2)?;
            let end_index = cx.argument::<JsValue>(3)?;
            Ok(JsSignedReport::new(
                &mut cx,
                vec![this, memo_type, memo_data, start_index, end_index],
            )?
            .upcast())
        }

        method toObject(mut cx) {
            let this = cx.this().borrow(&cx.lock()).clone();
            Ok(neon_serde::to_value(&mut cx, &this)?)
        }

        method toString(mut cx) {
            let debug_str = format!("{:?}", *cx.this().borrow(&cx.lock()));
            Ok(cx.string(debug_str).upcast())
        }

        method toJSON(mut cx) {
            let this = cx.this().borrow(&cx.lock()).clone();
            let json = serde_json::to_string(&this).unwrap();
            let string = cx.string(json);
            Ok(string.upcast())
        }
    }

    pub class JsTemporaryContactKey for TemporaryContactKey {
        init(mut cx) {
            let from_value = cx.argument::<JsValue>(0)?;
            if from_value.is_a::<JsReportAuthorizationKey>() {
                let rak = cx.argument::<JsReportAuthorizationKey>(0)?;
                let guard = cx.lock();
                let rak = rak.borrow(&guard);
                Ok(rak.initial_temporary_contact_key())
            } else if from_value.is_a::<JsTemporaryContactKey>() {
                let tck = cx.argument::<JsTemporaryContactKey>(0)?;
                let guard = cx.lock();
                let tck = tck.borrow(&guard);
                Ok(tck.ratchet().expect("No more TCNs available"))
            } else {
                Ok(neon_serde::from_value(&mut cx, from_value)?)
            }
        }

        method temporary_contact_number(mut cx) {
            let tcn = cx.this().borrow(&cx.lock()).temporary_contact_number();
            Ok(to_buffer(&mut cx, &tcn.0)?.upcast())
        }

        method ratchet(mut cx) {
            let this = cx.this();
            Ok(JsTemporaryContactKey::new(&mut cx, vec![this])?.upcast())
        }

        method toObject(mut cx) {
            let this = cx.this().borrow(&cx.lock()).clone();
            Ok(neon_serde::to_value(&mut cx, &this)?)
        }

        method toString(mut cx) {
            let debug_str = format!("{:?}", *cx.this().borrow(&cx.lock()));
            Ok(cx.string(debug_str).upcast())
        }

        method toJSON(mut cx) {
            let this = cx.this().borrow(&cx.lock()).clone();
            let json = serde_json::to_string(&this).unwrap();
            let string = cx.string(json);
            Ok(string.upcast())
        }
    }

    pub class JsSignedReport for SignedReport {
        init(mut cx) {
            let from_value = cx.argument::<JsValue>(0)?;
            if from_value.is_a::<JsReportAuthorizationKey>() {
                let rak = cx.argument::<JsReportAuthorizationKey>(0)?;
                let memo_type = match cx.argument::<JsNumber>(1)?.value() as u8 {
                    0 => MemoType::CoEpiV1,
                    1 => MemoType::CovidWatchV1,
                    0xff => MemoType::Reserved,
                    _ => panic!("Unknown memo type"),
                };
                let memo_data = cx.argument::<JsBuffer>(2)?;
                let start_index = cx.argument::<JsNumber>(3)?.value() as u16;
                let end_index = cx.argument::<JsNumber>(4)?.value() as u16;
                let guard = cx.lock();
                let rak = rak.borrow(&guard);
                let memo_data = memo_data.borrow(&guard);
                Ok(rak
                    .create_report(
                        memo_type,
                        memo_data.as_slice().to_vec(),
                        start_index,
                        end_index,
                    )
                    .expect("Memo data too long"))
            } else {
                Ok(neon_serde::from_value(&mut cx, from_value)?)
            }
        }

        method verify(mut cx) {
            let this = cx.this();
            Ok(JsReport::new(&mut cx, vec![this])?.upcast())
        }

        method toObject(mut cx) {
            let this = cx.this().borrow(&cx.lock()).clone();
            Ok(neon_serde::to_value(&mut cx, &this)?)
        }

        method toJSON(mut cx) {
            let this = cx.this().borrow(&cx.lock()).clone();
            let json = serde_json::to_string(&this).unwrap();
            let string = cx.string(json);
            Ok(string.upcast())
        }

        method toString(mut cx) {
            let debug_str = format!("{:?}", *cx.this().borrow(&cx.lock()));
            Ok(cx.string(debug_str).upcast())
        }
    }

    pub class JsReport for Report {
        init(mut cx) {
            let signed_report = cx.argument::<JsSignedReport>(0)?;
            let signed_report = signed_report.borrow(&cx.lock()).clone();
            Ok(signed_report.verify().expect("Invalid Signature"))
        }

        method temporary_contact_numbers(mut cx) {
            // collect all TCNs first because so that we know how many there are ...
            let tcns: Vec<_> = cx
                .this()
                .borrow(&cx.lock())
                .temporary_contact_numbers()
                .collect();
            // ... because JsArray must be initialized with a fixed length
            let array = JsArray::new(&mut cx, tcns.len() as u32);
            for (i, tcn) in tcns.iter().enumerate() {
                let buffer = to_buffer(&mut cx, &tcn.0)?;
                array.set(&mut cx, i as u32, buffer).unwrap();
            }
            Ok(array.upcast())
        }

        method toObject(mut cx) {
            let this = cx.this().borrow(&cx.lock()).clone();
            Ok(neon_serde::to_value(&mut cx, &this)?)
        }

        method toString(mut cx) {
            let debug_str = format!("{:?}", *cx.this().borrow(&cx.lock()));
            Ok(cx.string(debug_str).upcast())
        }

        method toJSON(mut cx) {
            let this = cx.this().borrow(&cx.lock()).clone();
            let json = serde_json::to_string(&this).unwrap();
            let string = cx.string(json);
            Ok(string.upcast())
        }
    }
}

register_module!(mut m, {
    m.export_class::<JsReportAuthorizationKey>("ReportAuthorizationKey")?;
    m.export_class::<JsTemporaryContactKey>("TemporaryContactKey")?;
    m.export_class::<JsSignedReport>("SignedReport")?;
    m.export_class::<JsReport>("Report")?;

    let co_epi_v1 = m.number(0);
    let covid_watch_v1 = m.number(1);
    let reserved = m.number(0xff);
    let memo_type = JsObject::new(&mut m);
    memo_type.set(&mut m, "CoEpiV1", co_epi_v1)?;
    memo_type.set(&mut m, "CovidWatchV1", covid_watch_v1)?;
    memo_type.set(&mut m, "Reserved", reserved)?;
    m.export_value("MemoType", memo_type)?;

    Ok(())
});
