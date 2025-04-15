use dap::types::PresentationHint;
use dap::types::StackFramePresentationhint;
use dap::types::Thread;
use std::cell::RefCell;

use crate::dap::requests::LaunchRequestArguments;
use crate::dap::responses::ThreadsResponse;
use crate::dap::Client;
use crate::dap::Service;
use crate::tracer::DebugTrace;

pub struct Debugger {
    debug_trace: RefCell<DebugTrace>,
    client: Client,
}

impl Debugger {
    pub fn new(client: Client, debug_trace: DebugTrace) -> Self {
        Self {
            client,
            debug_trace: RefCell::new(debug_trace),
        }
    }
}

impl Service for Debugger {
    fn launch(&self, _body: LaunchRequestArguments) {}

    fn threads(&self) -> ThreadsResponse {
        let resp = ThreadsResponse {
            threads: vec![Thread {
                id: 1,
                name: "Main Thread".to_string(),
            }],
        };

        self.client.send_event(dap::prelude::Event::Stopped(
            dap::events::StoppedEventBody {
                reason: dap::types::StoppedEventReason::Breakpoint,
                description: None,
                thread_id: Some(1),
                preserve_focus_hint: None,
                text: None,
                all_threads_stopped: Some(true),
                hit_breakpoint_ids: None,
            },
        ));

        resp
    }

    fn step_back(&self, _body: dap::requests::StepBackArguments) {
        self.debug_trace.borrow_mut().prev();

        self.client.send_event(dap::prelude::Event::Stopped(
            dap::events::StoppedEventBody {
                reason: dap::types::StoppedEventReason::Step,
                description: None,
                thread_id: Some(1),
                preserve_focus_hint: None,
                text: None,
                all_threads_stopped: Some(true),
                hit_breakpoint_ids: None,
            },
        ));
    }

    fn next(&self, _body: dap::requests::NextArguments) {
        self.debug_trace.borrow_mut().next();

        // just stop right away
        self.client.send_event(dap::prelude::Event::Stopped(
            dap::events::StoppedEventBody {
                reason: dap::types::StoppedEventReason::Step,
                description: None,
                thread_id: Some(1),
                preserve_focus_hint: None,
                text: None,
                all_threads_stopped: Some(true),
                hit_breakpoint_ids: None,
            },
        ));
    }

    fn scopes(&self, _body: dap::requests::ScopesArguments) -> dap::responses::ScopesResponse {
        let variables_in_scope = self.debug_trace.borrow().scope();

        dap::responses::ScopesResponse {
            scopes: variables_in_scope
                .into_iter()
                .map(|var| dap::types::Scope {
                    name: var.name,
                    presentation_hint: None,
                    variables_reference: var.id as i64,
                    ..Default::default()
                })
                .collect(), // Add appropriate scopes here
        }
    }

    fn stack_trace(
        &self,
        _body: dap::requests::StackTraceArguments,
    ) -> dap::responses::StackTraceResponse {
        let concrete_trace = self.debug_trace.borrow().trace();
        let len_frames = concrete_trace.stack_frames.len();

        let traces = concrete_trace
            .stack_frames
            .into_iter()
            .enumerate()
            .map(|(i, trace)| {
                let source_location = trace.location;
                dap::types::StackFrame {
                    id: i as i64,
                    name: format!("Frame {}", i),
                    line: source_location.line as i64,
                    column: (source_location.column + 1) as i64,
                    end_line: source_location.end_line.map(|l| l as i64),
                    end_column: source_location.end_column.map(|c| (c + 1) as i64),
                    source: Some(dap::types::Source {
                        name: Some(format!("Frame {}", i)),
                        path: Some(trace.path.clone()),
                        presentation_hint: Some(PresentationHint::Normal),
                        source_reference: None,
                        origin: None,
                        sources: None,
                        adapter_data: None,
                        checksums: None,
                    }),
                    presentation_hint: Some(StackFramePresentationhint::Normal), // Add presentation hint
                    ..Default::default()
                }
            })
            .collect();

        dap::responses::StackTraceResponse {
            stack_frames: traces,
            total_frames: Some(len_frames as i64),
        }
    }
}
