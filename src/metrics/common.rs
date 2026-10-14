use prometheus::{GaugeVec, IntGaugeVec, Opts, Registry};

pub fn gauge(registry: &Registry, name: &str, help: &str, labels: &[&str]) -> anyhow::Result<GaugeVec> {
    let g = GaugeVec::new(Opts::new(name, help), labels)?;
    registry.register(Box::new(g.clone()))?;
    Ok(g)
}

pub fn igauge(
    registry: &Registry,
    name: &str,
    help: &str,
    labels: &[&str],
) -> anyhow::Result<IntGaugeVec> {
    let g = IntGaugeVec::new(Opts::new(name, help), labels)?;
    registry.register(Box::new(g.clone()))?;
    Ok(g)
}

pub fn set(m: &GaugeVec, labels: &[&str], v: Option<f64>) {
    if let Some(v) = v {
        m.with_label_values(labels).set(v);
    }
}

pub fn set_i(m: &IntGaugeVec, labels: &[&str], v: i64) {
    m.with_label_values(labels).set(v);
}

pub fn ratio(v: Option<f64>) -> Option<f64> {
    v.map(|x| if x > 1.0 { x / 100.0 } else { x })
}

pub fn ms_to_sec(v: Option<f64>) -> Option<f64> {
    v.map(|x| x / 1000.0)
}

pub fn ccq_ratio(v: Option<f64>) -> Option<f64> {
    v.map(|x| {
        if x > 10.0 {
            x / 1000.0
        } else if x > 1.0 {
            x / 100.0
        } else {
            x
        }
    })
}
