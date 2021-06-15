// Copyright (c) Calibra Research
// SPDX-License-Identifier: Apache-2.0

#![cfg(feature = "simulator")]

use bft_lib::{
    base_types::*,
    simulated_context::{SimulatedContext, State},
    simulator,
};
use librabft_v2::{
    data_sync::*,
    node::{NodeConfig, NodeState},
};

type Context = SimulatedContext<NodeConfig>;

fn make_simulator(
    seed: u64,
    nodes: usize,
) -> simulator::Simulator<
    NodeState<Context>,
    Context,
    DataSyncNotification<Context>,
    DataSyncRequest,
    DataSyncResponse<Context>,
> {
    let context_factory = |author, num_nodes| {
        let config = NodeConfig {
            target_commit_interval: Duration(100000),
            delta: Duration(20),
            gamma_times_100: 200,
            lambda_times_100: 50,
        };
        SimulatedContext::new(author, config, num_nodes, 30000)
    };
    let delay_distribution = simulator::RandomDelay::new(10.0, 4.0);
    simulator::Simulator::new(seed, nodes, delay_distribution, context_factory)
}

#[test]
fn test_simulated_run_3_nodes() {
    let mut sim = make_simulator(/* seed */ 52, /* nodes */ 3);
    let contexts = sim.loop_until(simulator::GlobalTime(1000), None);
    let num_commits = contexts
        .iter()
        .map(|context| context.committed_history().len())
        .collect::<Vec<_>>();
    assert_eq!(num_commits, [27, 27, 27]);
    let last_committed_states = contexts
        .iter()
        .map(|context| context.last_committed_state())
        .collect::<Vec<_>>();
    assert_eq!(
        last_committed_states,
        [
            State(11134312813757838303),
            State(11134312813757838303),
            State(11134312813757838303)
        ],
    );
}

#[test]
fn test_simulated_run_8_nodes() {
    let mut sim = make_simulator(/* seed */ 48, /* nodes */ 8);
    let contexts = sim.loop_until(simulator::GlobalTime(1000), None);
    let num_commits = contexts
        .iter()
        .map(|context| context.committed_history().len())
        .collect::<Vec<_>>();
    assert_eq!(num_commits, [28, 28, 28, 28, 28, 28, 28, 30]);
    let last_committed_states = contexts
        .iter()
        .map(|context| context.last_committed_state())
        .collect::<Vec<_>>();
    assert_eq!(
        last_committed_states,
        [
            State(12785928431398617538),
            State(12785928431398617538),
            State(12785928431398617538),
            State(12785928431398617538),
            State(12785928431398617538),
            State(12785928431398617538),
            State(12785928431398617538),
            State(4890275890002623733)
        ]
    );
}