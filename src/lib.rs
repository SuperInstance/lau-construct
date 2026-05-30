//! # lau-construct — The Matrix Construct
//!
//! The shared creative space where user and agent co-create PLATO environments.
//! Like Neo and Trinity loading weapons, the user vibe-codes what they need and
//! the Construct materializes it as PLATO rooms, agent capabilities, hardware
//! controls, and game mechanics.

use serde::{Deserialize, Serialize};


// ---------------------------------------------------------------------------
// 1. ConstructId
// ---------------------------------------------------------------------------

/// Unique identifier for a Construct.
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConstructId(pub String);

impl ConstructId {
    pub fn new() -> Self {
        Self(format!("construct-{}", uuid()))
    }
}

impl Default for ConstructId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for ConstructId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ---------------------------------------------------------------------------
// 2. ConstructState
// ---------------------------------------------------------------------------

/// The current state of a Construct — mirrors the white-room lifecycle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ConstructState {
    /// White room, nothing loaded.
    #[default]
    Empty,
    /// Materializing the user's request.
    Loading,
    /// Everything loaded, waiting for walkthrough.
    Ready,
    /// User is walking through and testing.
    Testing,
    /// User is changing things.
    Modifying,
    /// Taking it live.
    Deploying,
    /// Deployed and running.
    Active,
}



impl std::fmt::Display for ConstructState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "Empty (white room)"),
            Self::Loading => write!(f, "Loading…"),
            Self::Ready => write!(f, "Ready"),
            Self::Testing => write!(f, "Testing"),
            Self::Modifying => write!(f, "Modifying"),
            Self::Deploying => write!(f, "Deploying…"),
            Self::Active => write!(f, "Active"),
        }
    }
}

// ---------------------------------------------------------------------------
// 3. RoomType
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RoomType {
    Training,
    Mission,
    Hardware,
    Bridge,
    Social,
    Creative,
    Custom(String),
}

impl std::fmt::Display for RoomType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Training => write!(f, "Training"),
            Self::Mission => write!(f, "Mission"),
            Self::Hardware => write!(f, "Hardware"),
            Self::Bridge => write!(f, "Bridge"),
            Self::Social => write!(f, "Social"),
            Self::Creative => write!(f, "Creative"),
            Self::Custom(s) => write!(f, "Custom({})", s),
        }
    }
}

// ---------------------------------------------------------------------------
// 4. BridgeStatus
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BridgeStatus {
    Connected,
    Pending,
    Simulated,
}

impl std::fmt::Display for BridgeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Connected => write!(f, "Connected"),
            Self::Pending => write!(f, "Pending"),
            Self::Simulated => write!(f, "Simulated"),
        }
    }
}

// ---------------------------------------------------------------------------
// 5. ConstructIntent
// ---------------------------------------------------------------------------

/// What the user actually wants, parsed from raw text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConstructIntent {
    NeedRooms(Vec<String>),
    NeedAgents(Vec<String>),
    NeedHardware(Vec<String>),
    NeedBridges(Vec<String>),
    NeedSkills(Vec<String>),
    NeedCustom(String),
    LoadProgram(String),
    Modify(String, String),
    Test(String),
    Deploy,
    Reset,
}

// ---------------------------------------------------------------------------
// 6. ConstructRequest
// ---------------------------------------------------------------------------

/// What the user says they need — raw text + parsed intent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstructRequest {
    pub raw_text: String,
    pub parsed_intent: ConstructIntent,
    pub voice: bool,
}

impl ConstructRequest {
    /// Parse natural language into a structured request.
    pub fn parse(text: &str) -> Self {
        let lower = text.to_lowercase();
        let intent = if lower.contains("deploy") || lower.contains("take it live") || lower.contains("go live") {
            ConstructIntent::Deploy
        } else if lower.contains("reset") || lower.contains("clear") || lower.contains("start over") {
            ConstructIntent::Reset
        } else if lower.contains("test") {
            ConstructIntent::Test(text.to_string())
        } else if lower.contains("make") || lower.contains("change") || lower.contains("modify")
            || lower.contains("bigger") || lower.contains("smaller")
        {
            ConstructIntent::Modify(text.to_string(), text.to_string())
        } else if lower.contains("load") && (lower.contains("program") || lower.contains("training")) {
            ConstructIntent::LoadProgram(text.to_string())
        } else if lower.contains("room") || lower.contains("chamber") || lower.contains("space") {
            ConstructIntent::NeedRooms(extract_items_for(text, "room"))
        } else if lower.contains("skill") || lower.contains("train") || lower.contains("learn") {
            ConstructIntent::NeedSkills(extract_items_for(text, "skill"))
        } else if lower.contains("agent") || lower.contains("operator") || lower.contains("gun") {
            ConstructIntent::NeedAgents(extract_items_for(text, "agent"))
        } else if lower.contains("motor") || lower.contains("servo") || lower.contains("sensor")
            || lower.contains("hardware")
        {
            ConstructIntent::NeedHardware(extract_items_for(text, "hardware"))
        } else if lower.contains("bridge") || lower.contains("connect") || lower.contains("link") {
            ConstructIntent::NeedBridges(extract_items_for(text, "bridge"))
        } else if lower.contains("skill") || lower.contains("train") || lower.contains("learn") {
            ConstructIntent::NeedSkills(extract_items(text))
        } else {
            ConstructIntent::NeedCustom(text.to_string())
        };

        Self {
            raw_text: text.to_string(),
            parsed_intent: intent,
            voice: false,
        }
    }

    pub fn is_voice(&self) -> bool {
        self.voice
    }
}

/// Extract items from a specific category only.
fn extract_items_for(text: &str, category: &str) -> Vec<String> {
    let lower = text.to_lowercase();
    let mut items = Vec::new();

    let keywords: &[&str] = match category {
        "room" => &[
            "engineering", "science", "training", "mission", "hardware",
            "bridge", "social", "creative", "combat", "medical", "command",
        ],
        "agent" => &[
            "scout", "diplomat", "engineer", "medic", "warrior", "pilot",
            "analyst", "guard", "operator",
        ],
        "hardware" => &[
            "servo", "motor", "sensor", "camera", "display", "speaker",
            "microphone", "led", "relay",
        ],
        "bridge" => &[
            "instance", "alpha", "beta", "gamma", "delta",
        ],
        "skill" => &[
            "gun-fu", "jujitsu", "judo", "karate", "piloting", "hacking",
            "medicine", "engineering", "diplomacy", "stealth",
            "combat", "driving", "flying",
        ],
        _ => &[],
    };

    for keyword in keywords {
        if lower.contains(keyword) && !items.iter().any(|i: &String| i.contains(keyword) || keyword.contains(i.as_str())) {
            items.push(keyword.to_string());
        }
    }

    if items.is_empty() {
        items.push(text.trim().to_string());
    }
    items
}

/// Extract meaningful items from freeform text — crude but functional.
fn extract_items(text: &str) -> Vec<String> {
    let lower = text.to_lowercase();
    let mut items = Vec::new();

    // Known room types
    for keyword in &[
        "engineering", "science", "training", "mission", "hardware",
        "bridge", "social", "creative", "combat", "medical", "command",
    ] {
        if lower.contains(keyword) {
            items.push(keyword.to_string());
        }
    }
    // Known agent types — skip if already matched as a substring of an existing item
    for keyword in &[
        "scout", "diplomat", "engineer", "medic", "warrior", "pilot",
        "analyst", "guard", "operator", "agent",
    ] {
        if lower.contains(keyword) && !items.iter().any(|i| i.contains(keyword)) {
            items.push(keyword.to_string());
        }
    }
    // Known hardware
    for keyword in &[
        "servo", "motor", "sensor", "camera", "display", "speaker",
        "microphone", "led", "relay",
    ] {
        if lower.contains(keyword) && !items.iter().any(|i| i.contains(keyword)) {
            items.push(keyword.to_string());
        }
    }
    // Known skills — skip "engineering" since it's a room type, skip dupes
    for keyword in &[
        "gun-fu", "jujitsu", "judo", "karate", "piloting", "hacking",
        "medicine", "diplomacy", "stealth",
    ] {
        if lower.contains(keyword) && !items.iter().any(|i| i.contains(keyword)) {
            items.push(keyword.to_string());
        }
    }

    if items.is_empty() {
        items.push(text.trim().to_string());
    }
    items
}

// ---------------------------------------------------------------------------
// 7. ConstructRoom
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstructRoom {
    pub id: String,
    pub name: String,
    pub room_type: RoomType,
    pub contents: Vec<String>,
    pub size: (usize, usize),
    pub energy_level: f64,
    pub connected_to: Vec<String>,
    pub loaded: bool,
}

impl ConstructRoom {
    pub fn new(name: &str, room_type: RoomType) -> Self {
        Self {
            id: format!("room-{}", uuid()),
            name: name.to_string(),
            room_type,
            contents: Vec::new(),
            size: (10, 10),
            energy_level: 0.0,
            connected_to: Vec::new(),
            loaded: false,
        }
    }

    /// Load / materialize this room into the Construct.
    pub fn materialize(&mut self) {
        self.loaded = true;
        self.energy_level = 1.0;
    }

    /// Describe what this room looks like in the Construct.
    pub fn describe(&self) -> String {
        if self.loaded {
            let mut desc = format!(
                "[{}] {} ({}x{}) — {}",
                self.room_type, self.name, self.size.0, self.size.1,
                if self.contents.is_empty() {
                    "empty".to_string()
                } else {
                    self.contents.join(", ")
                }
            );
            if !self.connected_to.is_empty() {
                desc.push_str(&format!(" → connected to: {}", self.connected_to.join(", ")));
            }
            desc
        } else {
            format!(
                "[{}] {} — not yet materialized",
                self.room_type, self.name
            )
        }
    }

    /// Render a textual representation.
    pub fn render(&self) -> String {
        if self.loaded {
            let (w, h) = self.size;
            let mut lines = vec![format!("╔{}╗", "═".repeat(w.max(2) - 2))];
            for _ in 0..h.saturating_sub(2).max(1) {
                lines.push(format!("║{}║", " ".repeat(w.max(2) - 2)));
            }
            lines.push(format!("╚{}╝", "═".repeat(w.max(2) - 2)));
            lines.insert(1, format!("║ {} ║", center_str(&self.name, w.max(2) - 4)));
            lines.join("\n")
        } else {
            format!("░░ {} ░░", self.name)
        }
    }
}

fn center_str(s: &str, width: usize) -> String {
    if width == 0 {
        return String::new();
    }
    if s.len() >= width {
        // Don't truncate — expand the display instead
        return s.to_string();
    }
    let pad = width.saturating_sub(s.len());
    let left = pad / 2;
    let right = pad - left;
    format!("{}{}{}", " ".repeat(left), s, " ".repeat(right))
}

// ---------------------------------------------------------------------------
// 8. ConstructAgent
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstructAgent {
    pub id: String,
    pub name: String,
    pub archetype: String,
    pub level: u32,
    pub skills: Vec<String>,
    pub equipment: Vec<String>,
    pub loaded: bool,
    pub voice: Option<String>,
}

impl ConstructAgent {
    pub fn new(name: &str, archetype: &str) -> Self {
        Self {
            id: format!("agent-{}", uuid()),
            name: name.to_string(),
            archetype: archetype.to_string(),
            level: 1,
            skills: Vec::new(),
            equipment: Vec::new(),
            loaded: false,
            voice: None,
        }
    }

    /// Teach the agent a new skill.
    pub fn learn_skill(&mut self, skill: &str) {
        if !self.skills.contains(&skill.to_string()) {
            self.skills.push(skill.to_string());
        }
        self.level = self.level.saturating_add(1);
    }

    /// Give the agent a piece of equipment.
    pub fn equip(&mut self, item: &str) {
        if !self.equipment.contains(&item.to_string()) {
            self.equipment.push(item.to_string());
        }
    }

    /// Describe what this agent looks like in the Construct.
    pub fn describe(&self) -> String {
        if self.loaded {
            let skills_str = if self.skills.is_empty() {
                "no skills loaded".to_string()
            } else {
                format!("skills: {}", self.skills.join(", "))
            };
            let equip_str = if self.equipment.is_empty() {
                "unequipped".to_string()
            } else {
                format!("equipped: {}", self.equipment.join(", "))
            };
            format!(
                "{} [{}] Lv{} — {}, {}{}",
                self.name,
                self.archetype,
                self.level,
                skills_str,
                equip_str,
                self.voice
                    .as_deref()
                    .map(|v| format!(" (voice: {})", v))
                    .unwrap_or_default()
            )
        } else {
            format!("{} [{}] — not yet loaded", self.name, self.archetype)
        }
    }
}

// ---------------------------------------------------------------------------
// 9. ConstructHardware
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstructHardware {
    pub id: String,
    pub name: String,
    pub hw_type: String,
    pub channels: u32,
    pub connected: bool,
    pub simulated: bool,
}

impl ConstructHardware {
    pub fn new(name: &str, hw_type: &str) -> Self {
        Self {
            id: format!("hw-{}", uuid()),
            name: name.to_string(),
            hw_type: hw_type.to_string(),
            channels: 1,
            connected: false,
            simulated: true,
        }
    }

    pub fn describe(&self) -> String {
        let status = if self.connected {
            "connected"
        } else if self.simulated {
            "simulated"
        } else {
            "offline"
        };
        format!(
            "{} ({}, {}ch) — {}",
            self.name, self.hw_type, self.channels, status
        )
    }
}

// ---------------------------------------------------------------------------
// 10. ConstructBridge
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstructBridge {
    pub id: String,
    pub target: String,
    pub status: BridgeStatus,
}

impl ConstructBridge {
    pub fn new(target: &str) -> Self {
        Self {
            id: format!("bridge-{}", uuid()),
            target: target.to_string(),
            status: BridgeStatus::Pending,
        }
    }

    pub fn describe(&self) -> String {
        format!("Bridge → {} [{}]", self.target, self.status)
    }
}

// ---------------------------------------------------------------------------
// 11. ConstructResult
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstructResult {
    pub success: bool,
    pub materialized: Vec<String>,
    pub message: String,
    pub energy_cost: f64,
    pub state_after: ConstructState,
}

// ---------------------------------------------------------------------------
// 12. ConstructView
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstructView {
    pub perspective: String,
    pub rooms_visible: Vec<String>,
    pub agents_visible: Vec<String>,
    pub hardware_visible: Vec<String>,
    pub narrative: String,
}

impl ConstructView {
    pub fn render(&self, mode: ViewMode) -> String {
        match mode {
            ViewMode::Matrix => {
                let mut lines = vec!["═══ The Construct ═══".to_string()];
                lines.push(String::new());
                if !self.rooms_visible.is_empty() {
                    lines.push("Rooms:".to_string());
                    for r in &self.rooms_visible {
                        lines.push(format!("  ■ {}", r));
                    }
                }
                if !self.agents_visible.is_empty() {
                    lines.push("Agents:".to_string());
                    for a in &self.agents_visible {
                        lines.push(format!("  ◆ {}", a));
                    }
                }
                if !self.hardware_visible.is_empty() {
                    lines.push("Hardware:".to_string());
                    for h in &self.hardware_visible {
                        lines.push(format!("  ● {}", h));
                    }
                }
                lines.push(String::new());
                lines.push(self.narrative.clone());
                lines.join("\n")
            }
            ViewMode::MUD => {
                let mut s = format!("You are {}.\n", self.perspective);
                if self.rooms_visible.is_empty() {
                    s.push_str("You stand in a white, featureless room.\n");
                } else {
                    s.push_str("Around you, the Construct materializes:\n");
                    for r in &self.rooms_visible {
                        s.push_str(&format!("  > {}\n", r));
                    }
                }
                if !self.agents_visible.is_empty() {
                    s.push_str("You see figures:\n");
                    for a in &self.agents_visible {
                        s.push_str(&format!("  > {}\n", a));
                    }
                }
                if !self.hardware_visible.is_empty() {
                    s.push_str("Equipment hums:\n");
                    for h in &self.hardware_visible {
                        s.push_str(&format!("  > {}\n", h));
                    }
                }
                s
            }
            ViewMode::Dashboard => {
                let mut lines = vec!["┌─── Construct Dashboard ───┐".to_string()];
                lines.push(format!(
                    "│ Rooms: {:<20} │",
                    self.rooms_visible.len().to_string()
                ));
                lines.push(format!(
                    "│ Agents: {:<19} │",
                    self.agents_visible.len().to_string()
                ));
                lines.push(format!(
                    "│ Hardware: {:<17} │",
                    self.hardware_visible.len().to_string()
                ));
                lines.push("└───────────────────────────┘".to_string());
                lines.push(self.narrative.clone());
                lines.join("\n")
            }
            ViewMode::Voice => self.narrative.clone(),
        }
    }
}

// ---------------------------------------------------------------------------
// 13. ViewMode / ConstructRenderMode
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ViewMode {
    Matrix,
    MUD,
    Dashboard,
    Voice,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConstructRenderMode {
    Matrix,
    MUD,
    JSON,
    Narrative,
    Voice,
}

// ---------------------------------------------------------------------------
// 14. ConstructStatus
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstructStatus {
    pub state: ConstructState,
    pub room_count: usize,
    pub agent_count: usize,
    pub hardware_count: usize,
    pub bridge_count: usize,
    pub energy_used: f64,
    pub energy_remaining: f64,
}

impl std::fmt::Display for ConstructStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Construct [{}] — {} rooms, {} agents, {} hardware, {} bridges | energy: {:.1} used / {:.1} remaining",
            self.state,
            self.room_count,
            self.agent_count,
            self.hardware_count,
            self.bridge_count,
            self.energy_used,
            self.energy_remaining
        )
    }
}

// ---------------------------------------------------------------------------
// 15. DeployResult
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployResult {
    pub success: bool,
    pub deployed_items: Vec<String>,
    pub energy_committed: f64,
    pub message: String,
}

// ---------------------------------------------------------------------------
// 16. Construct — THE shared space
// ---------------------------------------------------------------------------

/// The Matrix Construct — the room where user and agent co-create.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Construct {
    pub id: ConstructId,
    pub name: String,
    pub creator: String,
    pub operator: String,
    pub state: ConstructState,
    pub rooms: Vec<ConstructRoom>,
    pub agents: Vec<ConstructAgent>,
    pub hardware: Vec<ConstructHardware>,
    pub bridges: Vec<ConstructBridge>,
    pub energy_pool: f64,
    pub tick: u64,
    pub voice_enabled: bool,
    #[serde(skip)]
    energy_spent: f64,
}

impl Construct {
    /// Create a new Construct (white room).
    pub fn new(name: &str, creator: &str, operator: &str, budget: f64) -> Self {
        Self {
            id: ConstructId::new(),
            name: name.to_string(),
            creator: creator.to_string(),
            operator: operator.to_string(),
            state: ConstructState::Empty,
            rooms: Vec::new(),
            agents: Vec::new(),
            hardware: Vec::new(),
            bridges: Vec::new(),
            energy_pool: budget,
            tick: 0,
            voice_enabled: false,
            energy_spent: 0.0,
        }
    }

    /// Materialize what the user asked for.
    pub fn manifest(&mut self, request: &ConstructRequest) -> Vec<ConstructResult> {
        self.state = ConstructState::Loading;
        self.tick += 1;
        let mut results = Vec::new();

        match &request.parsed_intent {
            ConstructIntent::NeedRooms(names) => {
                for name in names {
                    let rt = guess_room_type(name);
                    let mut room = ConstructRoom::new(name, rt);
                    let cost = 10.0;
                    if self.can_afford(cost) {
                        room.materialize();
                        self.spend(cost);
                        let desc = room.describe();
                        self.rooms.push(room);
                        results.push(ConstructResult {
                            success: true,
                            materialized: vec![desc],
                            message: format!("Room \"{}\" materialized.", name),
                            energy_cost: cost,
                            state_after: ConstructState::Ready,
                        });
                    } else {
                        results.push(ConstructResult {
                            success: false,
                            materialized: vec![],
                            message: "Insufficient energy.".to_string(),
                            energy_cost: 0.0,
                            state_after: self.state.clone(),
                        });
                    }
                }
            }
            ConstructIntent::NeedAgents(names) => {
                for name in names {
                    let archetype = guess_archetype(name);
                    let mut agent = ConstructAgent::new(name, &archetype);
                    let cost = 15.0;
                    if self.can_afford(cost) {
                        agent.loaded = true;
                        let desc = agent.describe();
                        self.agents.push(agent);
                        results.push(ConstructResult {
                            success: true,
                            materialized: vec![desc],
                            message: format!("Agent \"{}\" loaded.", name),
                            energy_cost: cost,
                            state_after: ConstructState::Ready,
                        });
                        self.spend(cost);
                    } else {
                        results.push(ConstructResult {
                            success: false,
                            materialized: vec![],
                            message: "Insufficient energy.".to_string(),
                            energy_cost: 0.0,
                            state_after: self.state.clone(),
                        });
                    }
                }
            }
            ConstructIntent::NeedHardware(names) => {
                for name in names {
                    let hw_type = guess_hw_type(name);
                    let mut hw = ConstructHardware::new(name, &hw_type);
                    let cost = 5.0;
                    if self.can_afford(cost) {
                        hw.connected = true;
                        hw.simulated = false;
                        let desc = hw.describe();
                        self.hardware.push(hw);
                        results.push(ConstructResult {
                            success: true,
                            materialized: vec![desc],
                            message: format!("Hardware \"{}\" connected.", name),
                            energy_cost: cost,
                            state_after: ConstructState::Ready,
                        });
                        self.spend(cost);
                    } else {
                        results.push(ConstructResult {
                            success: false,
                            materialized: vec![],
                            message: "Insufficient energy.".to_string(),
                            energy_cost: 0.0,
                            state_after: self.state.clone(),
                        });
                    }
                }
            }
            ConstructIntent::NeedBridges(names) => {
                for name in names {
                    let cost = 8.0;
                    if self.can_afford(cost) {
                        let mut bridge = ConstructBridge::new(name);
                        bridge.status = BridgeStatus::Connected;
                        let desc = bridge.describe();
                        self.bridges.push(bridge);
                        results.push(ConstructResult {
                            success: true,
                            materialized: vec![desc],
                            message: format!("Bridge to \"{}\" established.", name),
                            energy_cost: cost,
                            state_after: ConstructState::Ready,
                        });
                        self.spend(cost);
                    } else {
                        results.push(ConstructResult {
                            success: false,
                            materialized: vec![],
                            message: "Insufficient energy.".to_string(),
                            energy_cost: 0.0,
                            state_after: self.state.clone(),
                        });
                    }
                }
            }
            ConstructIntent::NeedSkills(skills) => {
                for skill in skills {
                    if let Some(agent) = self.agents.last_mut() {
                        agent.learn_skill(skill);
                        results.push(ConstructResult {
                            success: true,
                            materialized: vec![format!("{} learned {}", agent.name, skill)],
                            message: format!("Skill \"{}\" uploaded.", skill),
                            energy_cost: 3.0,
                            state_after: ConstructState::Ready,
                        });
                        self.spend(3.0);
                    } else {
                        results.push(ConstructResult {
                            success: false,
                            materialized: vec![],
                            message: "No agent to learn skill. Load an agent first.".to_string(),
                            energy_cost: 0.0,
                            state_after: self.state.clone(),
                        });
                    }
                }
            }
            ConstructIntent::NeedCustom(desc) => {
                let cost = 12.0;
                if self.can_afford(cost) {
                    let mut room = ConstructRoom::new(desc, RoomType::Custom("custom".to_string()));
                    room.materialize();
                    let rdesc = room.describe();
                    self.rooms.push(room);
                    self.spend(cost);
                    results.push(ConstructResult {
                        success: true,
                        materialized: vec![rdesc],
                        message: format!("Custom construct \"{}\" materialized.", desc),
                        energy_cost: cost,
                        state_after: ConstructState::Ready,
                    });
                } else {
                    results.push(ConstructResult {
                        success: false,
                        materialized: vec![],
                        message: "Insufficient energy.".to_string(),
                        energy_cost: 0.0,
                        state_after: self.state.clone(),
                    });
                }
            }
            ConstructIntent::LoadProgram(name) => {
                let lower = name.to_lowercase();
                let prog = if lower.contains("combat") || lower.contains("gun") || lower.contains("fight") {
                    load_combat_program()
                } else if lower.contains("engineer") || lower.contains("motor") || lower.contains("hardware") {
                    load_engineering_program()
                } else if lower.contains("social") || lower.contains("diplom") || lower.contains("palaver") {
                    load_social_program()
                } else if lower.contains("explore") || lower.contains("scout") || lower.contains("map") {
                    load_exploration_program()
                } else {
                    load_combat_program()
                };
                self.rooms = prog.rooms;
                self.agents = prog.agents;
                self.hardware = prog.hardware;
                self.bridges = prog.bridges;
                self.energy_spent += 20.0;
                results.push(ConstructResult {
                    success: true,
                    materialized: self.describe_all(),
                    message: format!("Program \"{}\" loaded.", name),
                    energy_cost: 20.0,
                    state_after: ConstructState::Ready,
                });
            }
            ConstructIntent::Modify(item_id, change) => {
                let r = self.modify(item_id, change);
                results.push(r);
            }
            ConstructIntent::Test(_target) => {
                self.state = ConstructState::Testing;
                results.push(ConstructResult {
                    success: true,
                    materialized: self.describe_all(),
                    message: "Entering test mode. Walk through the Construct.".to_string(),
                    energy_cost: 0.0,
                    state_after: ConstructState::Testing,
                });
            }
            ConstructIntent::Deploy => {
                let deploy = self.deploy();
                if deploy.success {
                    self.state = ConstructState::Active;
                }
                results.push(ConstructResult {
                    success: deploy.success,
                    materialized: deploy.deployed_items.clone(),
                    message: deploy.message,
                    energy_cost: deploy.energy_committed,
                    state_after: self.state.clone(),
                });
            }
            ConstructIntent::Reset => {
                self.reset();
                results.push(ConstructResult {
                    success: true,
                    materialized: vec![],
                    message: "Construct cleared. White room.".to_string(),
                    energy_cost: 0.0,
                    state_after: ConstructState::Empty,
                });
            }
        }

        // Auto-transition state
        if self.state == ConstructState::Loading {
            self.state = ConstructState::Ready;
        }

        results
    }

    /// Remove something from the Construct.
    pub fn dematerialize(&mut self, item_id: &str) -> bool {
        if let Some(pos) = self.rooms.iter().position(|r| r.id == item_id || r.name == item_id) {
            self.rooms.remove(pos);
            return true;
        }
        if let Some(pos) = self.agents.iter().position(|a| a.id == item_id || a.name == item_id) {
            self.agents.remove(pos);
            return true;
        }
        if let Some(pos) = self.hardware.iter().position(|h| h.id == item_id || h.name == item_id) {
            self.hardware.remove(pos);
            return true;
        }
        if let Some(pos) = self.bridges.iter().position(|b| b.id == item_id || b.target == item_id) {
            self.bridges.remove(pos);
            return true;
        }
        false
    }

    /// Change something in the Construct.
    pub fn modify(&mut self, item_id: &str, change: &str) -> ConstructResult {
        self.state = ConstructState::Modifying;
        let lower = change.to_lowercase();

        // Try rooms
        for room in &mut self.rooms {
            if room.id == item_id || room.name == item_id {
                if lower.contains("bigger") || lower.contains("larger") {
                    room.size = (room.size.0 * 2, room.size.1 * 2);
                    return ConstructResult {
                        success: true,
                        materialized: vec![room.describe()],
                        message: format!("Room \"{}\" expanded to {}x{}.", room.name, room.size.0, room.size.1),
                        energy_cost: 2.0,
                        state_after: ConstructState::Ready,
                    };
                }
                if lower.contains("smaller") {
                    room.size = (room.size.0.max(4) / 2, room.size.1.max(4) / 2);
                    return ConstructResult {
                        success: true,
                        materialized: vec![room.describe()],
                        message: format!("Room \"{}\" shrunk to {}x{}.", room.name, room.size.0, room.size.1),
                        energy_cost: 1.0,
                        state_after: ConstructState::Ready,
                    };
                }
                room.contents.push(change.to_string());
                return ConstructResult {
                    success: true,
                    materialized: vec![room.describe()],
                    message: format!("Room \"{}\" modified: {}.", room.name, change),
                    energy_cost: 1.0,
                    state_after: ConstructState::Ready,
                };
            }
        }

        // Try agents
        for agent in &mut self.agents {
            if agent.id == item_id || agent.name == item_id {
                if lower.contains("skill") || lower.contains("learn") || lower.contains("train") {
                    agent.learn_skill(change);
                }
                agent.equip(change);
                return ConstructResult {
                    success: true,
                    materialized: vec![agent.describe()],
                    message: format!("Agent \"{}\" modified: {}.", agent.name, change),
                    energy_cost: 2.0,
                    state_after: ConstructState::Ready,
                };
            }
        }

        ConstructResult {
            success: false,
            materialized: vec![],
            message: format!("Item \"{}\" not found in Construct.", item_id),
            energy_cost: 0.0,
            state_after: self.state.clone(),
        }
    }

    /// Walk through the Construct from a perspective.
    pub fn walk_through(&self, viewer: &str) -> ConstructView {
        let rooms_visible: Vec<String> = self.rooms.iter().map(|r| r.describe()).collect();
        let agents_visible: Vec<String> = self.agents.iter().map(|a| a.describe()).collect();
        let hardware_visible: Vec<String> = self.hardware.iter().map(|h| h.describe()).collect();

        let mut narrative = format!("{} steps into the Construct.\n", viewer);
        if self.rooms.is_empty() && self.agents.is_empty() && self.hardware.is_empty() {
            narrative.push_str("The white room stretches infinitely in all directions. Nothing has been materialized yet.");
        } else {
            if !self.rooms.is_empty() {
                narrative.push_str(&format!("{} rooms shimmer into existence.\n", self.rooms.len()));
            }
            if !self.agents.is_empty() {
                narrative.push_str(&format!(
                    "{} agent{} stand{} ready.\n",
                    self.agents.len(),
                    if self.agents.len() == 1 { "" } else { "s" },
                    if self.agents.len() == 1 { "s" } else { "" }
                ));
            }
            if !self.hardware.is_empty() {
                narrative.push_str(&format!("{} piece{} of hardware hum{} in the background.\n",
                    self.hardware.len(),
                    if self.hardware.len() == 1 { "" } else { "s" },
                    if self.hardware.len() == 1 { "s" } else { "" }
                ));
            }
            if !self.bridges.is_empty() {
                narrative.push_str(&format!("{} bridge{} pulse{} with light.\n",
                    self.bridges.len(),
                    if self.bridges.len() == 1 { "" } else { "s" },
                    if self.bridges.len() == 1 { "s" } else { "" }
                ));
            }
        }

        ConstructView {
            perspective: viewer.to_string(),
            rooms_visible,
            agents_visible,
            hardware_visible,
            narrative,
        }
    }

    /// Take the Construct live.
    pub fn deploy(&self) -> DeployResult {
        let mut deployed = Vec::new();
        for r in &self.rooms {
            deployed.push(format!("room:{}", r.name));
        }
        for a in &self.agents {
            deployed.push(format!("agent:{}", a.name));
        }
        for h in &self.hardware {
            deployed.push(format!("hw:{}", h.name));
        }
        for b in &self.bridges {
            deployed.push(format!("bridge:{}", b.target));
        }

        DeployResult {
            success: !deployed.is_empty(),
            deployed_items: deployed.clone(),
            energy_committed: self.energy_spent,
            message: if deployed.is_empty() {
                "Nothing to deploy — Construct is empty.".to_string()
            } else {
                format!("Deployed {} items to PLATO.", deployed.len())
            },
        }
    }

    /// Clear the Construct back to white room.
    pub fn reset(&mut self) {
        self.rooms.clear();
        self.agents.clear();
        self.hardware.clear();
        self.bridges.clear();
        self.state = ConstructState::Empty;
        self.tick += 1;
    }

    /// Get the current status.
    pub fn status(&self) -> ConstructStatus {
        ConstructStatus {
            state: self.state.clone(),
            room_count: self.rooms.len(),
            agent_count: self.agents.len(),
            hardware_count: self.hardware.len(),
            bridge_count: self.bridges.len(),
            energy_used: self.energy_spent,
            energy_remaining: self.energy_remaining(),
        }
    }

    /// Render the full Construct in a given mode.
    pub fn render(&self, mode: ConstructRenderMode) -> String {
        match mode {
            ConstructRenderMode::Matrix => {
                let view = self.walk_through("Operator");
                view.render(ViewMode::Matrix)
            }
            ConstructRenderMode::MUD => {
                let view = self.walk_through("You");
                view.render(ViewMode::MUD)
            }
            ConstructRenderMode::JSON => serde_json::to_string_pretty(self).unwrap_or_default(),
            ConstructRenderMode::Narrative => {
                let view = self.walk_through(&self.creator);
                view.narrative
            }
            ConstructRenderMode::Voice => {
                let view = self.walk_through(&self.creator);
                view.render(ViewMode::Voice)
            }
        }
    }

    /// Check if energy is conserved (spent <= pool).
    pub fn is_conserved(&self) -> bool {
        self.energy_spent <= self.energy_pool + f64::EPSILON
    }

    // -- helpers --

    fn can_afford(&self, cost: f64) -> bool {
        self.energy_remaining() >= cost
    }

    fn spend(&mut self, cost: f64) {
        self.energy_spent += cost;
    }

    fn energy_remaining(&self) -> f64 {
        (self.energy_pool - self.energy_spent).max(0.0)
    }

    fn describe_all(&self) -> Vec<String> {
        let mut out = Vec::new();
        for r in &self.rooms { out.push(r.describe()); }
        for a in &self.agents { out.push(a.describe()); }
        for h in &self.hardware { out.push(h.describe()); }
        for b in &self.bridges { out.push(b.describe()); }
        out
    }
}

// ---------------------------------------------------------------------------
// 17. Pre-built programs
// ---------------------------------------------------------------------------

/// Combat training program — "I know gun-fu."
pub fn load_combat_program() -> Construct {
    let mut c = Construct::new("Combat Training", "system", "operator", 1000.0);
    let mut training = ConstructRoom::new("Dojo", RoomType::Training);
    training.materialize();
    training.size = (20, 20);
    training.contents = vec!["training mats".into(), "weapon racks".into(), "simulation targets".into()];
    let mut arsenal = ConstructRoom::new("Arsenal", RoomType::Training);
    arsenal.materialize();
    arsenal.contents = vec!["ranged weapons".into(), "melee weapons".into(), "explosives".into()];
    let mut arena = ConstructRoom::new("Arena", RoomType::Mission);
    arena.materialize();
    arena.size = (50, 50);
    arena.contents = vec!["combat arena".into(), "observation deck".into()];
    c.rooms = vec![training, arsenal, arena];

    let mut neo = ConstructAgent::new("Neo", "warrior");
    neo.loaded = true;
    neo.learn_skill("gun-fu");
    neo.learn_skill("jujitsu");
    neo.learn_skill("kendo");
    neo.equip("katana");
    neo.equip("pistol");

    let mut trinity = ConstructAgent::new("Trinity", "operator");
    trinity.loaded = true;
    trinity.learn_skill("hacking");
    trinity.learn_skill("piloting");
    trinity.equip("comm-link");

    c.agents = vec![neo, trinity];
    c.state = ConstructState::Ready;
    c
}

/// Engineering program — rooms for hardware control, motor agents, sensor arrays.
pub fn load_engineering_program() -> Construct {
    let mut c = Construct::new("Engineering Bay", "system", "operator", 1000.0);
    let mut lab = ConstructRoom::new("Hardware Lab", RoomType::Hardware);
    lab.materialize();
    lab.contents = vec!["oscilloscope".into(), "soldering station".into(), "logic analyzer".into()];

    let mut motor_bay = ConstructRoom::new("Motor Bay", RoomType::Hardware);
    motor_bay.materialize();
    motor_bay.contents = vec!["servo rack".into(), "controller board".into(), "power supply".into()];

    let mut sensor_room = ConstructRoom::new("Sensor Array", RoomType::Hardware);
    sensor_room.materialize();
    sensor_room.contents = vec!["lidar".into(), "camera array".into(), "proximity sensors".into()];

    c.rooms = vec![lab, motor_bay, sensor_room];

    let mut engineer = ConstructAgent::new("Spark", "engineer");
    engineer.loaded = true;
    engineer.learn_skill("circuit-design");
    engineer.learn_skill("firmware");
    engineer.equip("multimeter");
    engineer.equip("oscilloscope");

    let mut motor_op = ConstructAgent::new("Torque", "operator");
    motor_op.loaded = true;
    motor_op.learn_skill("motor-control");
    motor_op.learn_skill("pid-tuning");

    c.agents = vec![engineer, motor_op];

    let servo = {
        let mut s = ConstructHardware::new("Servo Array", "servo");
        s.channels = 6;
        s.connected = true;
        s.simulated = false;
        s
    };
    let sensor_hw = {
        let mut s = ConstructHardware::new("LIDAR Unit", "sensor");
        s.channels = 1;
        s.connected = true;
        s.simulated = false;
        s
    };
    c.hardware = vec![servo, sensor_hw];
    c.state = ConstructState::Ready;
    c
}

/// Social program — palaver rooms, bridges, diplomacy agents.
pub fn load_social_program() -> Construct {
    let mut c = Construct::new("Palaver Tree", "system", "operator", 1000.0);
    let mut palaver = ConstructRoom::new("Palaver Room", RoomType::Social);
    palaver.materialize();
    palaver.size = (30, 30);
    palaver.contents = vec!["circular seating".into(), "talking stick".into(), "consensus board".into()];

    let mut bridge_room = ConstructRoom::new("Bridge Room", RoomType::Bridge);
    bridge_room.materialize();
    bridge_room.contents = vec!["communication array".into(), "translation matrix".into()];

    c.rooms = vec![palaver, bridge_room];

    let mut diplomat = ConstructAgent::new("Eloquence", "diplomat");
    diplomat.loaded = true;
    diplomat.learn_skill("negotiation");
    diplomat.learn_skill("translation");
    diplomat.equip("protocol-manual");

    let mut herald = ConstructAgent::new("Herald", "operator");
    herald.loaded = true;
    herald.learn_skill("messaging");
    herald.learn_skill("broadcast");

    c.agents = vec![diplomat, herald];

    let mut b1 = ConstructBridge::new("instance-alpha");
    b1.status = BridgeStatus::Connected;
    let mut b2 = ConstructBridge::new("instance-beta");
    b2.status = BridgeStatus::Simulated;
    c.bridges = vec![b1, b2];
    c.state = ConstructState::Ready;
    c
}

/// Exploration program — scout agents, mapping rooms, terrain generators.
pub fn load_exploration_program() -> Construct {
    let mut c = Construct::new("Explorer's Guild", "system", "operator", 1000.0);
    let mut map_room = ConstructRoom::new("Cartography", RoomType::Creative);
    map_room.materialize();
    map_room.size = (40, 40);
    map_room.contents = vec!["terrain generator".into(), "map table".into(), "compass array".into()];

    let mut scout_den = ConstructRoom::new("Scout Den", RoomType::Mission);
    scout_den.materialize();
    scout_den.contents = vec!["field kits".into(), "communication gear".into()];

    c.rooms = vec![map_room, scout_den];

    let mut scout = ConstructAgent::new("Pathfinder", "scout");
    scout.loaded = true;
    scout.learn_skill("mapping");
    scout.learn_skill("stealth");
    scout.learn_skill("survival");
    scout.equip("map-kit");
    scout.equip("field-rations");

    let mut cartographer = ConstructAgent::new("Chart", "analyst");
    cartographer.loaded = true;
    cartographer.learn_skill("cartography");
    cartographer.learn_skill("terrain-analysis");
    cartographer.equip("survey-tools");

    c.agents = vec![scout, cartographer];

    let terrain_sensor = {
        let mut s = ConstructHardware::new("Terrain Scanner", "sensor");
        s.channels = 4;
        s.connected = true;
        s.simulated = false;
        s
    };
    c.hardware = vec![terrain_sensor];
    c.state = ConstructState::Ready;
    c
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn uuid() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let dur = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let base = dur.as_nanos() as u64;
    // Add a simple counter-like offset for uniqueness in tests
    static_counter(base)
}

static mut COUNTER: u64 = 0;
fn static_counter(base: u64) -> u64 {
    unsafe {
        COUNTER = COUNTER.wrapping_add(1);
        base.wrapping_add(COUNTER)
    }
}

fn guess_room_type(name: &str) -> RoomType {
    let lower = name.to_lowercase();
    if lower.contains("train") || lower.contains("dojo") || lower.contains("combat") {
        RoomType::Training
    } else if lower.contains("mission") || lower.contains("quest") {
        RoomType::Mission
    } else if lower.contains("hardware") || lower.contains("motor") || lower.contains("lab") {
        RoomType::Hardware
    } else if lower.contains("bridge") || lower.contains("link") {
        RoomType::Bridge
    } else if lower.contains("social") || lower.contains("palaver") {
        RoomType::Social
    } else if lower.contains("creative") || lower.contains("art") {
        RoomType::Creative
    } else {
        RoomType::Custom(name.to_string())
    }
}

fn guess_archetype(name: &str) -> String {
    let lower = name.to_lowercase();
    if lower.contains("warrior") || lower.contains("soldier") || lower.contains("fighter") {
        "warrior".to_string()
    } else if lower.contains("scout") || lower.contains("explorer") {
        "scout".to_string()
    } else if lower.contains("diplomat") || lower.contains("ambassador") {
        "diplomat".to_string()
    } else if lower.contains("engineer") || lower.contains("tech") {
        "engineer".to_string()
    } else if lower.contains("medic") || lower.contains("healer") {
        "medic".to_string()
    } else if lower.contains("pilot") {
        "pilot".to_string()
    } else if lower.contains("analyst") {
        "analyst".to_string()
    } else if lower.contains("guard") {
        "guard".to_string()
    } else {
        "operator".to_string()
    }
}

fn guess_hw_type(name: &str) -> String {
    let lower = name.to_lowercase();
    if lower.contains("servo") || lower.contains("motor") {
        "servo".to_string()
    } else if lower.contains("sensor") || lower.contains("lidar") || lower.contains("proximity") {
        "sensor".to_string()
    } else if lower.contains("camera") {
        "camera".to_string()
    } else if lower.contains("display") || lower.contains("screen") {
        "display".to_string()
    } else if lower.contains("speaker") || lower.contains("audio") {
        "speaker".to_string()
    } else if lower.contains("microphone") || lower.contains("mic") {
        "microphone".to_string()
    } else if lower.contains("led") || lower.contains("light") {
        "led".to_string()
    } else if lower.contains("relay") {
        "relay".to_string()
    } else {
        "generic".to_string()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -- ConstructId --
    #[test]
    fn construct_id_is_unique() {
        let a = ConstructId::new();
        let b = ConstructId::new();
        assert_ne!(a, b);
    }

    #[test]
    fn construct_id_display() {
        let id = ConstructId("test-42".to_string());
        assert_eq!(format!("{}", id), "test-42");
    }

    #[test]
    fn construct_id_clone_hash_eq() {
        let a = ConstructId("x".to_string());
        let b = a.clone();
        assert_eq!(a, b);
        let mut h1 = std::collections::HashSet::new();
        h1.insert(a.clone());
        assert!(h1.contains(&b));
    }

    #[test]
    fn construct_id_default() {
        let id = ConstructId::default();
        assert!(id.0.starts_with("construct-"));
    }

    #[test]
    fn construct_id_serde_roundtrip() {
        let id = ConstructId("round-trip".to_string());
        let json = serde_json::to_string(&id).unwrap();
        let back: ConstructId = serde_json::from_str(&json).unwrap();
        assert_eq!(id, back);
    }

    // -- ConstructState --
    #[test]
    fn state_default_is_empty() {
        assert_eq!(ConstructState::default(), ConstructState::Empty);
    }

    #[test]
    fn state_display() {
        assert_eq!(format!("{}", ConstructState::Empty), "Empty (white room)");
        assert_eq!(format!("{}", ConstructState::Active), "Active");
        assert_eq!(format!("{}", ConstructState::Loading), "Loading…");
    }

    #[test]
    fn state_serde_roundtrip() {
        for state in [
            ConstructState::Empty,
            ConstructState::Loading,
            ConstructState::Ready,
            ConstructState::Testing,
            ConstructState::Modifying,
            ConstructState::Deploying,
            ConstructState::Active,
        ] {
            let json = serde_json::to_string(&state).unwrap();
            let back: ConstructState = serde_json::from_str(&json).unwrap();
            assert_eq!(state, back);
        }
    }

    // -- Construct new --
    #[test]
    fn construct_new() {
        let c = Construct::new("Test", "neo", "operator", 100.0);
        assert_eq!(c.name, "Test");
        assert_eq!(c.creator, "neo");
        assert_eq!(c.operator, "operator");
        assert_eq!(c.state, ConstructState::Empty);
        assert_eq!(c.energy_pool, 100.0);
        assert!(c.rooms.is_empty());
        assert!(c.agents.is_empty());
        assert!(c.hardware.is_empty());
        assert!(c.bridges.is_empty());
        assert!(c.is_conserved());
    }

    // -- manifest rooms --
    #[test]
    fn manifest_rooms() {
        let mut c = Construct::new("T", "u", "a", 100.0);
        let req = ConstructRequest::parse("I need rooms for engineering and training");
        let results = c.manifest(&req);
        assert!(results.iter().all(|r| r.success));
        assert_eq!(c.rooms.len(), 2);
        assert!(c.rooms.iter().all(|r| r.loaded));
        assert_eq!(c.state, ConstructState::Ready);
    }

    // -- manifest agents --
    #[test]
    fn manifest_agents() {
        let mut c = Construct::new("T", "u", "a", 200.0);
        let req = ConstructRequest::parse("I need agents, lots of agents");
        let results = c.manifest(&req);
        assert!(results.iter().all(|r| r.success));
        assert!(!c.agents.is_empty());
        assert!(c.agents.iter().all(|a| a.loaded));
    }

    // -- manifest hardware --
    #[test]
    fn manifest_hardware() {
        let mut c = Construct::new("T", "u", "a", 100.0);
        let req = ConstructRequest::parse("I need motor controls for 3 servos");
        let results = c.manifest(&req);
        assert!(results.iter().all(|r| r.success));
        assert!(!c.hardware.is_empty());
    }

    // -- manifest bridges --
    #[test]
    fn manifest_bridges() {
        let mut c = Construct::new("T", "u", "a", 100.0);
        let req = ConstructRequest::parse("I need connections to the other instances via bridge");
        let results = c.manifest(&req);
        assert!(results.iter().all(|r| r.success));
        assert!(!c.bridges.is_empty());
    }

    // -- manifest skills --
    #[test]
    fn manifest_skills_no_agent() {
        let mut c = Construct::new("T", "u", "a", 100.0);
        let req = ConstructRequest::parse("I need gun-fu training for my agent");
        let results = c.manifest(&req);
        // No agent loaded yet
        assert!(results.iter().any(|r| !r.success));
    }

    #[test]
    fn manifest_skills_with_agent() {
        let mut c = Construct::new("T", "u", "a", 100.0);
        // First load an agent
        let load = ConstructRequest::parse("I need a warrior agent");
        c.manifest(&load);
        // Then teach skill
        let req = ConstructRequest::parse("I need gun-fu training for my agent");
        let results = c.manifest(&req);
        assert!(results.iter().all(|r| r.success));
        assert!(c.agents.last().unwrap().skills.contains(&"gun-fu".to_string()));
    }

    // -- manifest custom --
    #[test]
    fn manifest_custom() {
        let mut c = Construct::new("T", "u", "a", 100.0);
        let req = ConstructRequest::parse("I need a fractal geometry visualization space");
        let results = c.manifest(&req);
        assert!(results.iter().all(|r| r.success));
        assert_eq!(c.rooms.len(), 1);
    }

    // -- manifest test mode --
    #[test]
    fn manifest_test() {
        let mut c = Construct::new("T", "u", "a", 100.0);
        let load = ConstructRequest::parse("I need rooms for engineering");
        c.manifest(&load);
        let req = ConstructRequest::parse("test the engineering room");
        let results = c.manifest(&req);
        assert!(results[0].success);
        assert_eq!(c.state, ConstructState::Testing);
    }

    // -- manifest deploy --
    #[test]
    fn manifest_deploy() {
        let mut c = Construct::new("T", "u", "a", 100.0);
        let load = ConstructRequest::parse("I need rooms for engineering");
        c.manifest(&load);
        let req = ConstructRequest::parse("deploy");
        let results = c.manifest(&req);
        assert!(results[0].success);
        assert_eq!(c.state, ConstructState::Active);
    }

    #[test]
    fn deploy_empty_fails() {
        let c = Construct::new("T", "u", "a", 100.0);
        let result = c.deploy();
        assert!(!result.success);
        assert!(result.deployed_items.is_empty());
    }

    // -- manifest reset --
    #[test]
    fn manifest_reset() {
        let mut c = Construct::new("T", "u", "a", 100.0);
        let load = ConstructRequest::parse("I need rooms for engineering");
        c.manifest(&load);
        assert!(!c.rooms.is_empty());
        let req = ConstructRequest::parse("reset everything start over");
        c.manifest(&req);
        assert!(c.rooms.is_empty());
        assert_eq!(c.state, ConstructState::Empty);
    }

    // -- dematerialize --
    #[test]
    fn dematerialize_room() {
        let mut c = Construct::new("T", "u", "a", 100.0);
        let req = ConstructRequest::parse("I need rooms for training");
        c.manifest(&req);
        assert_eq!(c.rooms.len(), 1);
        let name = c.rooms[0].name.clone();
        assert!(c.dematerialize(&name));
        assert!(c.rooms.is_empty());
    }

    #[test]
    fn dematerialize_agent() {
        let mut c = Construct::new("T", "u", "a", 100.0);
        let req = ConstructRequest::parse("I need a scout agent");
        c.manifest(&req);
        assert_eq!(c.agents.len(), 1);
        let name = c.agents[0].name.clone();
        assert!(c.dematerialize(&name));
        assert!(c.agents.is_empty());
    }

    #[test]
    fn dematerialize_not_found() {
        let mut c = Construct::new("T", "u", "a", 100.0);
        assert!(!c.dematerialize("nonexistent"));
    }

    // -- modify --
    #[test]
    fn modify_room_bigger() {
        let mut c = Construct::new("T", "u", "a", 100.0);
        let req = ConstructRequest::parse("I need rooms for engineering");
        c.manifest(&req);
        let name = c.rooms[0].name.clone();
        let result = c.modify(&name, "make it bigger");
        assert!(result.success);
        assert!(c.rooms[0].size.0 > 10);
    }

    #[test]
    fn modify_room_smaller() {
        let mut c = Construct::new("T", "u", "a", 100.0);
        let req = ConstructRequest::parse("I need rooms for engineering");
        c.manifest(&req);
        let name = c.rooms[0].name.clone();
        let result = c.modify(&name, "make it smaller");
        assert!(result.success);
        assert!(c.rooms[0].size.0 < 10);
    }

    #[test]
    fn modify_agent_equip() {
        let mut c = Construct::new("T", "u", "a", 100.0);
        let req = ConstructRequest::parse("I need a warrior agent");
        c.manifest(&req);
        let name = c.agents[0].name.clone();
        let result = c.modify(&name, "plasma rifle");
        assert!(result.success);
        assert!(c.agents[0].equipment.contains(&"plasma rifle".to_string()));
    }

    #[test]
    fn modify_not_found() {
        let mut c = Construct::new("T", "u", "a", 100.0);
        let result = c.modify("ghost", "change");
        assert!(!result.success);
    }

    // -- walk_through --
    #[test]
    fn walk_through_empty() {
        let c = Construct::new("T", "neo", "a", 100.0);
        let view = c.walk_through("neo");
        assert_eq!(view.perspective, "neo");
        assert!(view.rooms_visible.is_empty());
        assert!(view.narrative.contains("white room"));
    }

    #[test]
    fn walk_through_loaded() {
        let mut c = Construct::new("T", "neo", "a", 200.0);
        c.manifest(&ConstructRequest::parse("I need rooms for training"));
        c.manifest(&ConstructRequest::parse("I need a warrior agent"));
        let view = c.walk_through("neo");
        assert_eq!(view.rooms_visible.len(), 1);
        assert_eq!(view.agents_visible.len(), 1);
        assert!(view.narrative.contains("rooms shimmer"));
    }

    // -- ViewMode rendering --
    #[test]
    fn view_render_matrix() {
        let c = Construct::new("T", "neo", "a", 200.0);
        let view = c.walk_through("neo");
        let rendered = view.render(ViewMode::Matrix);
        assert!(rendered.contains("The Construct"));
    }

    #[test]
    fn view_render_mud() {
        let c = Construct::new("T", "neo", "a", 200.0);
        let view = c.walk_through("neo");
        let rendered = view.render(ViewMode::MUD);
        assert!(rendered.contains("You are"));
    }

    #[test]
    fn view_render_dashboard() {
        let c = Construct::new("T", "neo", "a", 200.0);
        let view = c.walk_through("neo");
        let rendered = view.render(ViewMode::Dashboard);
        assert!(rendered.contains("Dashboard"));
    }

    #[test]
    fn view_render_voice() {
        let c = Construct::new("T", "neo", "a", 200.0);
        let view = c.walk_through("neo");
        let rendered = view.render(ViewMode::Voice);
        assert!(!rendered.is_empty());
    }

    // -- status --
    #[test]
    fn status_empty() {
        let c = Construct::new("T", "u", "a", 100.0);
        let s = c.status();
        assert_eq!(s.state, ConstructState::Empty);
        assert_eq!(s.room_count, 0);
        assert_eq!(s.energy_remaining, 100.0);
        assert_eq!(s.energy_used, 0.0);
    }

    #[test]
    fn status_display() {
        let c = Construct::new("T", "u", "a", 100.0);
        let s = c.status();
        let display = format!("{}", s);
        assert!(display.contains("Construct"));
        assert!(display.contains("Empty (white room)"));
    }

    // -- render modes --
    #[test]
    fn render_matrix_mode() {
        let c = Construct::new("T", "u", "a", 100.0);
        let s = c.render(ConstructRenderMode::Matrix);
        assert!(s.contains("The Construct"));
    }

    #[test]
    fn render_mud_mode() {
        let c = Construct::new("T", "u", "a", 100.0);
        let s = c.render(ConstructRenderMode::MUD);
        assert!(s.contains("You are"));
    }

    #[test]
    fn render_json_mode() {
        let c = Construct::new("T", "u", "a", 100.0);
        let s = c.render(ConstructRenderMode::JSON);
        let parsed: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(parsed["name"], "T");
    }

    #[test]
    fn render_narrative_mode() {
        let c = Construct::new("T", "u", "a", 100.0);
        let s = c.render(ConstructRenderMode::Narrative);
        assert!(s.contains("steps into the Construct"));
    }

    #[test]
    fn render_voice_mode() {
        let c = Construct::new("T", "u", "a", 100.0);
        let s = c.render(ConstructRenderMode::Voice);
        assert!(!s.is_empty());
    }

    // -- energy conservation --
    #[test]
    fn energy_conserved() {
        let mut c = Construct::new("T", "u", "a", 100.0);
        c.manifest(&ConstructRequest::parse("I need rooms for engineering"));
        assert!(c.is_conserved());
    }

    #[test]
    fn energy_depleted() {
        let mut c = Construct::new("T", "u", "a", 10.0);
        let results = c.manifest(&ConstructRequest::parse("I need rooms for engineering"));
        // Room costs 10, so first one succeeds
        assert!(results[0].success);
        // Second request should fail
        let results2 = c.manifest(&ConstructRequest::parse("I need rooms for science"));
        assert!(results2.iter().any(|r| !r.success));
    }

    // -- reset --
    #[test]
    fn reset_clears_everything() {
        let mut c = Construct::new("T", "u", "a", 200.0);
        c.manifest(&ConstructRequest::parse("I need rooms for engineering"));
        c.manifest(&ConstructRequest::parse("I need a warrior agent"));
        c.reset();
        assert!(c.rooms.is_empty());
        assert!(c.agents.is_empty());
        assert_eq!(c.state, ConstructState::Empty);
    }

    // -- ConstructRoom --
    #[test]
    fn room_new() {
        let r = ConstructRoom::new("TestRoom", RoomType::Training);
        assert_eq!(r.name, "TestRoom");
        assert_eq!(r.room_type, RoomType::Training);
        assert!(!r.loaded);
        assert_eq!(r.size, (10, 10));
    }

    #[test]
    fn room_materialize() {
        let mut r = ConstructRoom::new("TestRoom", RoomType::Training);
        r.materialize();
        assert!(r.loaded);
        assert_eq!(r.energy_level, 1.0);
    }

    #[test]
    fn room_describe_loaded() {
        let mut r = ConstructRoom::new("Dojo", RoomType::Training);
        r.materialize();
        r.contents.push("mats".to_string());
        let desc = r.describe();
        assert!(desc.contains("Dojo"));
        assert!(desc.contains("mats"));
    }

    #[test]
    fn room_describe_not_loaded() {
        let r = ConstructRoom::new("Dojo", RoomType::Training);
        let desc = r.describe();
        assert!(desc.contains("not yet materialized"));
    }

    #[test]
    fn room_render_loaded() {
        let mut r = ConstructRoom::new("TestRoom", RoomType::Training);
        r.materialize();
        let rendered = r.render();
        assert!(rendered.contains("TestRoom"));
        assert!(rendered.contains("╗"));
    }

    #[test]
    fn room_render_not_loaded() {
        let r = ConstructRoom::new("Ghost", RoomType::Training);
        let rendered = r.render();
        assert!(rendered.contains("░░"));
    }

    // -- ConstructAgent --
    #[test]
    fn agent_new() {
        let a = ConstructAgent::new("Neo", "warrior");
        assert_eq!(a.name, "Neo");
        assert_eq!(a.archetype, "warrior");
        assert_eq!(a.level, 1);
        assert!(a.skills.is_empty());
        assert!(!a.loaded);
    }

    #[test]
    fn agent_learn_skill() {
        let mut a = ConstructAgent::new("Neo", "warrior");
        a.learn_skill("gun-fu");
        assert!(a.skills.contains(&"gun-fu".to_string()));
        assert_eq!(a.level, 2);
        // learning same skill again shouldn't duplicate
        a.learn_skill("gun-fu");
        assert_eq!(a.skills.iter().filter(|s| **s == "gun-fu").count(), 1);
    }

    #[test]
    fn agent_equip() {
        let mut a = ConstructAgent::new("Neo", "warrior");
        a.equip("katana");
        assert!(a.equipment.contains(&"katana".to_string()));
        a.equip("katana"); // no duplicates
        assert_eq!(a.equipment.len(), 1);
    }

    #[test]
    fn agent_describe_loaded() {
        let mut a = ConstructAgent::new("Neo", "warrior");
        a.loaded = true;
        a.learn_skill("gun-fu");
        a.equip("katana");
        a.voice = Some("Nova".to_string());
        let desc = a.describe();
        assert!(desc.contains("Neo"));
        assert!(desc.contains("gun-fu"));
        assert!(desc.contains("katana"));
        assert!(desc.contains("Nova"));
    }

    #[test]
    fn agent_describe_not_loaded() {
        let a = ConstructAgent::new("Neo", "warrior");
        let desc = a.describe();
        assert!(desc.contains("not yet loaded"));
    }

    // -- ConstructHardware --
    #[test]
    fn hardware_new() {
        let h = ConstructHardware::new("Servo1", "servo");
        assert_eq!(h.name, "Servo1");
        assert_eq!(h.hw_type, "servo");
        assert!(!h.connected);
        assert!(h.simulated);
    }

    #[test]
    fn hardware_describe_simulated() {
        let h = ConstructHardware::new("Servo1", "servo");
        let desc = h.describe();
        assert!(desc.contains("simulated"));
    }

    #[test]
    fn hardware_describe_connected() {
        let mut h = ConstructHardware::new("Servo1", "servo");
        h.connected = true;
        let desc = h.describe();
        assert!(desc.contains("connected"));
    }

    // -- ConstructBridge --
    #[test]
    fn bridge_new() {
        let b = ConstructBridge::new("instance-alpha");
        assert_eq!(b.target, "instance-alpha");
        assert_eq!(b.status, BridgeStatus::Pending);
    }

    #[test]
    fn bridge_describe() {
        let b = ConstructBridge::new("instance-alpha");
        let desc = b.describe();
        assert!(desc.contains("instance-alpha"));
        assert!(desc.contains("Pending"));
    }

    // -- ConstructRequest parsing --
    #[test]
    fn request_parse_rooms() {
        let req = ConstructRequest::parse("I need rooms for engineering");
        assert!(matches!(req.parsed_intent, ConstructIntent::NeedRooms(_)));
    }

    #[test]
    fn request_parse_agents() {
        let req = ConstructRequest::parse("I need agents, lots of agents");
        assert!(matches!(req.parsed_intent, ConstructIntent::NeedAgents(_)));
    }

    #[test]
    fn request_parse_hardware() {
        let req = ConstructRequest::parse("I need motor controls");
        assert!(matches!(req.parsed_intent, ConstructIntent::NeedHardware(_)));
    }

    #[test]
    fn request_parse_bridges() {
        let req = ConstructRequest::parse("I need a bridge connection");
        assert!(matches!(req.parsed_intent, ConstructIntent::NeedBridges(_)));
    }

    #[test]
    fn request_parse_skills() {
        let req = ConstructRequest::parse("I need gun-fu training");
        assert!(matches!(req.parsed_intent, ConstructIntent::NeedSkills(_)));
    }

    #[test]
    fn request_parse_deploy() {
        let req = ConstructRequest::parse("deploy everything take it live");
        assert!(matches!(req.parsed_intent, ConstructIntent::Deploy));
    }

    #[test]
    fn request_parse_reset() {
        let req = ConstructRequest::parse("clear everything start over");
        assert!(matches!(req.parsed_intent, ConstructIntent::Reset));
    }

    #[test]
    fn request_parse_test() {
        let req = ConstructRequest::parse("test the room");
        assert!(matches!(req.parsed_intent, ConstructIntent::Test(_)));
    }

    #[test]
    fn request_parse_modify() {
        let req = ConstructRequest::parse("make the room bigger");
        assert!(matches!(req.parsed_intent, ConstructIntent::Modify(_, _)));
    }

    #[test]
    fn request_parse_load_program() {
        let req = ConstructRequest::parse("load the combat training program");
        assert!(matches!(req.parsed_intent, ConstructIntent::LoadProgram(_)));
    }

    #[test]
    fn request_parse_custom() {
        let req = ConstructRequest::parse("something totally weird and new");
        assert!(matches!(req.parsed_intent, ConstructIntent::NeedCustom(_)));
    }

    #[test]
    fn request_is_voice() {
        let mut req = ConstructRequest::parse("I need rooms");
        assert!(!req.is_voice());
        req.voice = true;
        assert!(req.is_voice());
    }

    // -- Pre-built programs --
    #[test]
    fn combat_program() {
        let c = load_combat_program();
        assert_eq!(c.name, "Combat Training");
        assert_eq!(c.rooms.len(), 3);
        assert_eq!(c.agents.len(), 2);
        assert!(c.agents[0].skills.contains(&"gun-fu".to_string()));
        assert_eq!(c.state, ConstructState::Ready);
    }

    #[test]
    fn engineering_program() {
        let c = load_engineering_program();
        assert_eq!(c.name, "Engineering Bay");
        assert_eq!(c.rooms.len(), 3);
        assert_eq!(c.agents.len(), 2);
        assert_eq!(c.hardware.len(), 2);
        assert!(c.hardware[0].connected);
        assert_eq!(c.state, ConstructState::Ready);
    }

    #[test]
    fn social_program() {
        let c = load_social_program();
        assert_eq!(c.name, "Palaver Tree");
        assert_eq!(c.rooms.len(), 2);
        assert_eq!(c.agents.len(), 2);
        assert_eq!(c.bridges.len(), 2);
        assert_eq!(c.state, ConstructState::Ready);
    }

    #[test]
    fn exploration_program() {
        let c = load_exploration_program();
        assert_eq!(c.name, "Explorer's Guild");
        assert_eq!(c.rooms.len(), 2);
        assert_eq!(c.agents.len(), 2);
        assert_eq!(c.hardware.len(), 1);
        assert_eq!(c.state, ConstructState::Ready);
    }

    #[test]
    fn load_combat_via_manifest() {
        let mut c = Construct::new("T", "u", "a", 1000.0);
        let req = ConstructRequest::parse("load the combat program");
        let results = c.manifest(&req);
        assert!(results[0].success);
        assert_eq!(c.rooms.len(), 3);
        assert_eq!(c.agents.len(), 2);
    }

    #[test]
    fn load_engineering_via_manifest() {
        let mut c = Construct::new("T", "u", "a", 1000.0);
        let req = ConstructRequest::parse("load the engineering program");
        let results = c.manifest(&req);
        assert!(results[0].success);
        assert!(!c.hardware.is_empty());
    }

    // -- Serde round-trip for full Construct --
    #[test]
    fn construct_serde_roundtrip() {
        let mut c = Construct::new("Test", "neo", "operator", 200.0);
        c.manifest(&ConstructRequest::parse("I need rooms for engineering"));
        c.manifest(&ConstructRequest::parse("I need a warrior agent"));
        let json = serde_json::to_string(&c).unwrap();
        let back: Construct = serde_json::from_str(&json).unwrap();
        assert_eq!(c.name, back.name);
        assert_eq!(c.rooms.len(), back.rooms.len());
        assert_eq!(c.agents.len(), back.agents.len());
    }

    // -- RoomType / BridgeStatus display & serde --
    #[test]
    fn room_type_display() {
        assert_eq!(format!("{}", RoomType::Training), "Training");
        assert_eq!(format!("{}", RoomType::Custom("foo".to_string())), "Custom(foo)");
    }

    #[test]
    fn bridge_status_display() {
        assert_eq!(format!("{}", BridgeStatus::Connected), "Connected");
        assert_eq!(format!("{}", BridgeStatus::Pending), "Pending");
        assert_eq!(format!("{}", BridgeStatus::Simulated), "Simulated");
    }

    // -- DeployResult --
    #[test]
    fn deploy_result_fields() {
        let dr = DeployResult {
            success: true,
            deployed_items: vec!["room:Dojo".to_string()],
            energy_committed: 10.0,
            message: "Deployed 1 items.".to_string(),
        };
        assert!(dr.success);
        assert_eq!(dr.deployed_items.len(), 1);
    }

    // -- ConstructResult --
    #[test]
    fn construct_result_fields() {
        let cr = ConstructResult {
            success: true,
            materialized: vec!["item".to_string()],
            message: "ok".to_string(),
            energy_cost: 5.0,
            state_after: ConstructState::Ready,
        };
        assert!(cr.success);
        assert_eq!(cr.energy_cost, 5.0);
    }

    // -- Full integration: Neo workflow --
    #[test]
    fn neo_workflow() {
        let mut c = Construct::new("The Construct", "neo", "operator", 500.0);
        // "I need guns, lots of guns"
        let r1 = c.manifest(&ConstructRequest::parse("load the combat training program"));
        assert!(r1[0].success);
        assert!(c.is_conserved());

        // Walk through
        let view = c.walk_through("Neo");
        assert!(!view.rooms_visible.is_empty());

        // Teach a skill
        let r2 = c.manifest(&ConstructRequest::parse("I need gun-fu training for my agent"));
        assert!(r2.iter().all(|r| r.success));

        // Modify
        let dojo_name = c.rooms[0].name.clone();
        let r3 = c.modify(&dojo_name, "make it bigger");
        assert!(r3.success);

        // Test
        let r4 = c.manifest(&ConstructRequest::parse("test the combat room"));
        assert!(r4[0].success);
        assert_eq!(c.state, ConstructState::Testing);

        // Deploy
        let r5 = c.manifest(&ConstructRequest::parse("deploy"));
        assert!(r5[0].success);
        assert_eq!(c.state, ConstructState::Active);

        // Status
        let status = c.status();
        assert_eq!(status.state, ConstructState::Active);
        assert!(status.room_count > 0);

        // Render
        let narrative = c.render(ConstructRenderMode::Narrative);
        assert!(!narrative.is_empty());

        // Reset
        c.reset();
        assert_eq!(c.state, ConstructState::Empty);
        assert!(c.rooms.is_empty());
    }
}
