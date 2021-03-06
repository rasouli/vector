package metadata

components: sinks: nats: {
	title:       "NATS"
	description: "[NATS.io](\(urls.nats)) is a simple, secure and high performance open source messaging system for cloud native applications, IoT messaging, and microservices architectures. NATS.io is a Cloud Native Computing Foundation project."

	classes: {
		commonly_used: false
		delivery:      "best_effort"
		development:   "beta"
		egress_method: "stream"
		service_providers: []
	}

	features: {
		buffer: enabled:      false
		healthcheck: enabled: true
		send: {
			compression: enabled: false
			encoding: {
				enabled: true
				codec: {
					enabled: true
					default: null
					enum: ["json", "text"]
				}
			}
			request: enabled: false
			tls: enabled:     false
			to: {
				service: {
					name:     "NATS"
					thing:    "a \(name) server"
					url:      urls.nats
					versions: null
				}

				interface: {
					socket: {
						direction: "outgoing"
						protocols: ["tcp"]
						ssl: "disabled"
					}
				}
			}
		}
	}

	support: {
		platforms: {
			"aarch64-unknown-linux-gnu":  true
			"aarch64-unknown-linux-musl": true
			"x86_64-apple-darwin":        true
			"x86_64-pc-windows-msv":      true
			"x86_64-unknown-linux-gnu":   true
			"x86_64-unknown-linux-musl":  true
		}

		requirements: []
		warnings: []
		notices: []
	}

	configuration: {
		url: {
			description: "The NATS URL to connect to. The url _must_ take the form of `nats://server:port`."
			groups: ["tcp"]
			required: true
			warnings: []
			type: string: {
				examples: ["nats://demo.nats.io", "nats://127.0.0.1:4222"]
			}
		}
		subject: {
			description: "The NATS subject to publish messages to."
			required:    true
			warnings: []
			type: string: {
				examples: ["{{ host }}", "foo", "time.us.east", "time.*.east", "time.>", ">"]
				templateable: true
			}
		}
		name: {
			common:      false
			description: "A name assigned to the NATS connection."
			required:    false
			type: string: {
				default: "vector"
				examples: ["foo", "API Name Option Example"]
			}
		}
	}

	input: {
		logs:    true
		metrics: null
	}
}
