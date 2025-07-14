use libp2p::{
    gossipsub::{Gossipsub, GossipsubEvent, MessageAuthenticity, ValidationMode},
    identity,
    mdns::{Mdns, MdnsEvent},
    swarm::NetworkBehaviourEventProcess,
    NetworkBehaviour, PeerId, Swarm, swarm::SwarmBuilder,
};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use tokio::io;

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "MyBehaviourEvent")]
pub struct MyBehaviour {
    pub gossipsub: Gossipsub,
    pub mdns: Mdns,
}

impl NetworkBehaviourEventProcess<MdnsEvent> for MyBehaviour {
    fn inject_event(&mut self, event: MdnsEvent) {
        if let MdnsEvent::Discovered(list) = event {
            for (peer, _) in list {
                self.gossipsub.add_explicit_peer(&peer);
            }
        }
    }
}

impl NetworkBehaviourEventProcess<GossipsubEvent> for MyBehaviour {
    fn inject_event(&mut self, event: GossipsubEvent) {
        if let GossipsubEvent::Message {
            propagation_source: peer_id,
            message_id: id,
            message,
        } = event
        {
            println!(
                "Got message: {} with id: {} from peer: {:?}",
                String::from_utf8_lossy(&message.data),
                id,
                peer_id
            );
        }
    }
}

pub enum MyBehaviourEvent {
    Gossipsub(GossipsubEvent),
    Mdns(MdnsEvent),
}

impl From<MdnsEvent> for MyBehaviourEvent {
    fn from(v: MdnsEvent) -> Self {
        MyBehaviourEvent::Mdns(v)
    }
}

impl From<GossipsubEvent> for MyBehaviourEvent {
    fn from(v: GossipsubEvent) -> Self {
        MyBehaviourEvent::Gossipsub(v)
    }
}

pub async fn create_swarm() -> Result<Swarm<MyBehaviour>, Box<dyn std::error::Error>> {
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());

    let transport = libp2p::development_transport(local_key.clone()).await?;

    let message_id_fn = |message: &libp2p::gossipsub::GossipsubMessage| {
        let mut s = DefaultHasher::new();
        message.data.hash(&mut s);
        libp2p::gossipsub::MessageId::from(s.finish().to_string())
    };

    let gossipsub_config = libp2p::gossipsub::GossipsubConfigBuilder::default()
        .heartbeat_interval(std::time::Duration::from_secs(10))
        .validation_mode(ValidationMode::Strict)
        .message_id_fn(message_id_fn)
        .build()
        .expect("Valid config");

    let mut gossipsub: Gossipsub =
        Gossipsub::new(MessageAuthenticity::Signed(local_key), gossipsub_config)
            .expect("Correct configuration");

    let mdns = Mdns::new(Default::default()).await?;
    let behaviour = MyBehaviour { gossipsub, mdns };
    let mut swarm = SwarmBuilder::new(transport, behaviour, local_peer_id)
        .executor(Box::new(|fut| {
            tokio::spawn(fut);
        }))
        .build();

    let listen_addr = "/ip4/0.0.0.0/tcp/0".parse()?;
    swarm.listen_on(listen_addr)?;

    Ok(swarm)
}
