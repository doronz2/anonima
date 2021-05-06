// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use libp2p::gossipsub::{
    score_parameter_decay, PeerScoreParams, PeerScoreThresholds, TopicScoreParams,
};
use std::collections::HashMap;
use std::time::Duration;

// All these parameters are copied from what Lotus has set for their Topic
// scores. They are currently unused because enabling them causes GossipSub
// blocks to come delayed usually by 1 second compared to when we have these
// parameters disabled. Leaving these here so that we can enable and fix these
// parameters when they are needed.

#[allow(dead_code)]
fn build_msg_topic_config() -> TopicScoreParams {
    TopicScoreParams {
        // expected 10 blocks/min
        topic_weight: 0.1,

        // 1 tick per second, maxes at 1 after 1 hour
        time_in_mesh_weight: 0.00027,
        time_in_mesh_quantum: Duration::from_secs(1),
        time_in_mesh_cap: 1.0,

        // deliveries decay after 10min, cap at 100 tx
        first_message_deliveries_weight: 5.0,
        first_message_deliveries_decay: score_parameter_decay(Duration::from_secs(10 * 60)), /* 10mins */
        // 100 blocks in an hour
        first_message_deliveries_cap: 100.0,
        // invalid messages decay after 1 hour
        invalid_message_deliveries_weight: -1000.0,
        invalid_message_deliveries_decay: score_parameter_decay(Duration::from_secs(60 * 60)),
        ..Default::default()
    }
}

#[allow(dead_code)]
fn build_block_topic_config() -> TopicScoreParams {
    TopicScoreParams {
        topic_weight: 0.1,

        // 1 tick per second, maxes at 1 hours (-1/3600)
        time_in_mesh_weight: 0.0002778,
        time_in_mesh_quantum: Duration::from_secs(1),
        time_in_mesh_cap: 1.0,

        // deliveries decay after 10min, cap at 100 tx
        first_message_deliveries_weight: 0.5,
        first_message_deliveries_decay: score_parameter_decay(Duration::from_secs(10 * 60)), /* 10mins */
        // 100 messages in 10 minutes
        first_message_deliveries_cap: 100.0,
        // invalid messages decay after 1 hour
        invalid_message_deliveries_weight: -1000.0,
        invalid_message_deliveries_decay: score_parameter_decay(Duration::from_secs(60 * 60)),
        ..Default::default()
    }
}

#[allow(dead_code)]
pub(crate) fn build_peer_score_params(network_name: &str) -> PeerScoreParams {
    let mut psp_topics = HashMap::new();

    PeerScoreParams {
        app_specific_weight: 1.0,

        ip_colocation_factor_threshold: 5.0,
        ip_colocation_factor_weight: -100.0,

        behaviour_penalty_threshold: 6.0,
        behaviour_penalty_weight: -10.0,
        behaviour_penalty_decay: score_parameter_decay(Duration::from_secs(60 * 60)),

        retain_score: Duration::from_secs(6 * 60 * 60),
        topics: psp_topics,
        ..Default::default()
    }
}

#[allow(dead_code)]
pub(crate) fn build_peer_score_threshold() -> PeerScoreThresholds {
    PeerScoreThresholds {
        gossip_threshold: -500.0,
        publish_threshold: -1000.0,
        graylist_threshold: -2500.0,
        accept_px_threshold: 1000.0,
        opportunistic_graft_threshold: 3.5,
    }
}
