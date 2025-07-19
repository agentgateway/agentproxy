use std::fmt::Debug;

use agent_core::metrics::{DefaultedUnknown, EncodeDisplay};
use agent_core::strng::RichStrng;
use agent_core::version;
use prometheus_client::encoding::EncodeLabelSet;
use prometheus_client::metrics::family::Family;
use prometheus_client::metrics::info::Info;
use prometheus_client::registry;
use prometheus_client::registry::Registry;

use crate::types::agent::BindProtocol;

#[derive(Clone, Hash, Default, Debug, PartialEq, Eq, EncodeLabelSet)]
pub struct HTTPLabels {
	pub bind: DefaultedUnknown<RichStrng>,
	pub gateway: DefaultedUnknown<RichStrng>,
	pub listener: DefaultedUnknown<RichStrng>,
	pub route: DefaultedUnknown<RichStrng>,
	pub route_rule: DefaultedUnknown<RichStrng>,
	pub backend: DefaultedUnknown<RichStrng>,

	pub method: DefaultedUnknown<EncodeDisplay<http::Method>>,
	pub status: DefaultedUnknown<EncodeDisplay<u16>>,
}

#[derive(Clone, Hash, Debug, PartialEq, Eq, EncodeLabelSet)]
pub struct TCPLabels {
	pub bind: DefaultedUnknown<RichStrng>,
	pub gateway: DefaultedUnknown<RichStrng>,
	pub listener: DefaultedUnknown<RichStrng>,
	pub protocol: BindProtocol,
}

type Counter = Family<HTTPLabels, prometheus_client::metrics::counter::Counter>;
type TCPCounter = Family<TCPLabels, prometheus_client::metrics::counter::Counter>;

#[derive(Clone, Hash, Debug, PartialEq, Eq, EncodeLabelSet)]
pub struct BuildLabel {
	tag: String,
}

#[derive(Debug)]
pub struct Metrics {
	pub requests: Counter,
	pub downstream_connection: TCPCounter,
}

impl Metrics {
	pub fn new(registry: &mut Registry) -> Self {
		registry.register(
			"build",
			"Agentgateway build information",
			Info::new(BuildLabel {
				tag: version::BuildInfo::new().git_tag,
			}),
		);
		Metrics {
			requests: build(
				registry,
				"requests",
				"The total number of HTTP requests sent",
			),
			downstream_connection: build(
				registry,
				"downstream_connections",
				"The total number of downstream connections established",
			),
		}
	}
}

fn build<T: Clone + std::hash::Hash + Eq + Send + Sync + Debug + EncodeLabelSet + 'static>(
	registry: &mut Registry,
	name: &str,
	help: &str,
) -> Family<T, prometheus_client::metrics::counter::Counter> {
	let m = Family::<T, _>::default();
	registry.register(name, help, m.clone());
	m
}
