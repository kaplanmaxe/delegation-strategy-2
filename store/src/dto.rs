pub struct Validator {
    pub identity: String,
    pub vote_account: String,
    pub epoch: i64,
    pub name: Option<String>,
    pub url: Option<String>,
    pub keybase: Option<String>,
    pub dc_ip: String,
    pub dc_coordinates_lat: Option<f64>,
    pub dc_coordinates_lon: Option<f64>,
    pub dc_continent: Option<String>,
    pub dc_country_iso: Option<String>,
    pub dc_country: Option<String>,
    pub dc_city: Option<String>,
    pub dc_asn: Option<i32>,
    pub dc_aso: Option<String>,
    pub max_commission: Option<u64>,
    pub version: Option<String>,
    pub mnde_votes: Option<i64>,
    pub activated_stake: i64,
    pub marinade_stake: i64,
    pub decentralizer_stake: i64,
    pub superminority: bool,
    pub stake_to_become_superminority: i64,
    pub credits: i64,
    pub leader_slots: usize,
    pub blocks_produced: usize,
    pub uptime_pct: Option<f64>,
    pub uptime: Option<i64>,
    pub downtime: Option<i64>,
}
