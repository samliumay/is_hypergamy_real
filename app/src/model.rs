//! The Hypergamy Meter scoring model.
//!
//! A tiny, fully transparent additive model: start from a base score, add one
//! signed contribution per input, clamp to `0..=100`, and map the result to a
//! [`Tier`]. Every function here is pure, so the whole model is unit-tested.
//!
//! # The weights are illustrative, not estimates
//!
//! ```text
//! ------------------------------------------------------------------------
//!  NONE OF THESE NUMBERS WERE ESTIMATED FROM DATA.
//!  They are hand-picked to make a joke app behave in a way that loosely
//!  echoes the *direction* of a few findings in the literature:
//!
//!  * Stated-preference studies (e.g. Buss 1989, 37 cultures) find that
//!    women say they value a partner's earning prospects more than men do,
//!    but the gap is modest, and speed-dating work (Eastwick & Finkel 2008)
//!    finds it shrinks further in actual choices. So income moves the score
//!    for both genders, a bit more for men, and never dominates.
//!  * Educational hypergamy (women "marrying up" in education) has weakened
//!    or reversed in many countries now that women out-graduate men
//!    (e.g. Esteve et al. 2016). Fewer same-or-higher educated men means the
//!    "market" for a highly educated woman who insists on marrying up is
//!    modelled as tighter, more so in regions where the norm of marrying up
//!    is still strong, and less so where hypogamy has become ordinary.
//!  * Age effects: the age at which stated desirability peaks tends to be
//!    lower for women than for men in online-dating data. The bands below
//!    copy that shape, gently.
//!
//!  * Looks, build, smarts and size (literature_review.md, section 11):
//!    self-rated looks get the largest weight for both sexes (Eastwick et
//!    al. 2014; Hitsch et al. 2010), build helps men more (Sell et al.
//!    2017), intelligence helps up to a ceiling near the 90th percentile
//!    (Gignac & Starbuck 2019), and penis size is a near-zero joke weight
//!    (Mautz et al. 2013; Lever et al. 2006: most partners are satisfied).
//!  * The relationship goal reweights everything: short-term choices lean on
//!    looks and build, long-term ones on status and smarts (Kenrick et al.
//!    1990; Li & Kenrick 2006).
//!  * Ethnicity is deliberately not an input: the studies on it document
//!    discrimination, and a score would repeat it (review, section 11.8).
//!
//!  Real effect sizes are small, noisy, culture-specific and contested.
//!  This model is for fun only. Not science, not advice.
//! ------------------------------------------------------------------------
//! ```

use std::fmt;

/// Score every profile starts from before any input is applied.
pub const BASE_SCORE: f32 = 60.0;

/// Lowest and highest score the meter can show.
pub const SCORE_RANGE: (f32, f32) = (0.0, 100.0);

/// Youngest age the app accepts (it is a dating-market joke, adults only).
pub const MIN_AGE: u8 = 18;

/// Oldest age the app accepts.
pub const MAX_AGE: u8 = 120;

/// Accepted height range in centimetres (only used when a height is given).
pub const HEIGHT_RANGE_CM: (u16, u16) = (100, 250);

/// Gender as entered by the user.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Gender {
    /// Woman.
    Woman,
    /// Man.
    Man,
    /// Non-binary. The joke model has no data, so it uses the midpoint of the
    /// woman and man weights for every input.
    NonBinary,
}

impl Gender {
    /// Every variant, in display order.
    pub const ALL: [Gender; 3] = [Gender::Woman, Gender::Man, Gender::NonBinary];

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Gender::Woman => "Woman",
            Gender::Man => "Man",
            Gender::NonBinary => "Non-binary",
        }
    }
}

/// Highest completed education level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Education {
    /// High school or less.
    HighSchool,
    /// Bachelor's degree.
    Bachelor,
    /// Master's degree.
    Master,
    /// Doctorate (yes, the researcher is aware).
    Doctorate,
}

impl Education {
    /// Every variant, in display order.
    pub const ALL: [Education; 4] = [
        Education::HighSchool,
        Education::Bachelor,
        Education::Master,
        Education::Doctorate,
    ];

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Education::HighSchool => "High school",
            Education::Bachelor => "Bachelor's",
            Education::Master => "Master's",
            Education::Doctorate => "PhD",
        }
    }
}

/// Income band relative to the user's own country.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IncomeBand {
    /// Bottom quarter.
    Low,
    /// Around the median.
    Middle,
    /// Comfortably above the median.
    UpperMiddle,
    /// Top few percent.
    High,
}

impl IncomeBand {
    /// Every variant, in display order.
    pub const ALL: [IncomeBand; 4] = [
        IncomeBand::Low,
        IncomeBand::Middle,
        IncomeBand::UpperMiddle,
        IncomeBand::High,
    ];

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            IncomeBand::Low => "Low (ramen era)",
            IncomeBand::Middle => "Middle",
            IncomeBand::UpperMiddle => "Upper middle",
            IncomeBand::High => "High (owns a boat?)",
        }
    }
}

/// Country or region, from a deliberately short list.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Region {
    /// Nordic countries and the Netherlands.
    NorthernEurope,
    /// Mediterranean Europe.
    SouthernEurope,
    /// Turkey and the Middle East.
    TurkeyMiddleEast,
    /// United States and Canada.
    NorthAmerica,
    /// Latin America.
    LatinAmerica,
    /// China, Japan, Korea.
    EastAsia,
}

impl Region {
    /// Every variant, in display order.
    pub const ALL: [Region; 6] = [
        Region::NorthernEurope,
        Region::SouthernEurope,
        Region::TurkeyMiddleEast,
        Region::NorthAmerica,
        Region::LatinAmerica,
        Region::EastAsia,
    ];

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Region::NorthernEurope => "Northern Europe",
            Region::SouthernEurope => "Southern Europe",
            Region::TurkeyMiddleEast => "Turkey / Middle East",
            Region::NorthAmerica => "North America",
            Region::LatinAmerica => "Latin America",
            Region::EastAsia => "East Asia",
        }
    }

    /// How strongly the local "marry up in education" norm amplifies the
    /// education contribution. Illustrative only: higher where the norm is
    /// described as stronger, lower where hypogamy has become common.
    pub const fn education_norm_factor(self) -> f32 {
        match self {
            Region::NorthernEurope => 0.6,
            Region::NorthAmerica => 0.9,
            Region::LatinAmerica => 1.0,
            Region::SouthernEurope => 1.1,
            Region::TurkeyMiddleEast => 1.3,
            Region::EastAsia => 1.5,
        }
    }
}

/// What the user is looking for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Goal {
    /// Something casual.
    Casual,
    /// A long-term relationship.
    LongTerm,
    /// Marriage.
    Marriage,
    /// No idea yet.
    Undecided,
}

impl Goal {
    /// Every variant, in display order.
    pub const ALL: [Goal; 4] = [
        Goal::Casual,
        Goal::LongTerm,
        Goal::Marriage,
        Goal::Undecided,
    ];

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Goal::Casual => "Casual",
            Goal::LongTerm => "Long-term",
            Goal::Marriage => "Marriage",
            Goal::Undecided => "No idea, honestly",
        }
    }
}

/// Self-rated looks. Self-rated, so mostly a confidence test.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Looks {
    /// Below average (by your own harsh standards).
    BelowAverage,
    /// Average.
    Average,
    /// Above average.
    AboveAverage,
    /// Turns heads (or so your mother says).
    HeadTurner,
}

impl Looks {
    /// Every variant, in display order.
    pub const ALL: [Looks; 4] = [
        Looks::BelowAverage,
        Looks::Average,
        Looks::AboveAverage,
        Looks::HeadTurner,
    ];

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Looks::BelowAverage => "Below average (harsh mirror)",
            Looks::Average => "Average",
            Looks::AboveAverage => "Above average",
            Looks::HeadTurner => "Head-turner (says my mum)",
        }
    }
}

/// Body build, self-rated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Build {
    /// Professional couch tester.
    CouchPotato,
    /// Average.
    Average,
    /// Goes to the gym on purpose.
    GymRegular,
    /// Carved from marble.
    GreekStatue,
}

impl Build {
    /// Every variant, in display order.
    pub const ALL: [Build; 4] = [
        Build::CouchPotato,
        Build::Average,
        Build::GymRegular,
        Build::GreekStatue,
    ];

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Build::CouchPotato => "Couch potato",
            Build::Average => "Average",
            Build::GymRegular => "Gym regular",
            Build::GreekStatue => "Greek statue",
        }
    }
}

/// Self-rated intelligence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Smarts {
    /// Average.
    Average,
    /// Smart.
    Smart,
    /// Very smart.
    VerySmart,
    /// Past the ceiling: the bonus shrinks again.
    GalaxyBrain,
}

impl Smarts {
    /// Every variant, in display order.
    pub const ALL: [Smarts; 4] = [
        Smarts::Average,
        Smarts::Smart,
        Smarts::VerySmart,
        Smarts::GalaxyBrain,
    ];

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Smarts::Average => "Average",
            Smarts::Smart => "Smart",
            Smarts::VerySmart => "Very smart",
            Smarts::GalaxyBrain => "Galaxy brain",
        }
    }
}

/// Penis size, self-reported, men only, and an obvious joke input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Size {
    /// The default: no answer, no effect.
    PreferNotToSay,
    /// Below average.
    BelowAverage,
    /// Average.
    Average,
    /// Above average.
    AboveAverage,
    /// Legendary (self-reported, naturally).
    Legendary,
}

impl Size {
    /// Every variant, in display order.
    pub const ALL: [Size; 5] = [
        Size::PreferNotToSay,
        Size::BelowAverage,
        Size::Average,
        Size::AboveAverage,
        Size::Legendary,
    ];

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Size::PreferNotToSay => "Prefer not to say",
            Size::BelowAverage => "Below average",
            Size::Average => "Average",
            Size::AboveAverage => "Above average",
            Size::Legendary => "Legendary (self-reported)",
        }
    }
}

/// Everything the user enters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Profile {
    /// Age in years.
    pub age: u8,
    /// Gender.
    pub gender: Gender,
    /// Highest completed education.
    pub education: Education,
    /// Income band.
    pub income: IncomeBand,
    /// Height in centimetres, if the user wants the height joke.
    pub height_cm: Option<u16>,
    /// Self-rated looks.
    pub looks: Looks,
    /// Self-rated build.
    pub build: Build,
    /// Self-rated intelligence.
    pub smarts: Smarts,
    /// Penis size (men only; ignored for women).
    pub size: Size,
    /// Country or region.
    pub region: Region,
    /// Relationship goal.
    pub goal: Goal,
}

impl Default for Profile {
    fn default() -> Self {
        Profile {
            age: 28,
            gender: Gender::Woman,
            education: Education::Bachelor,
            income: IncomeBand::Middle,
            height_cm: None,
            looks: Looks::Average,
            build: Build::Average,
            smarts: Smarts::Average,
            size: Size::PreferNotToSay,
            region: Region::TurkeyMiddleEast,
            goal: Goal::LongTerm,
        }
    }
}

/// Why a profile could not be scored.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProfileError {
    /// Age below [`MIN_AGE`] or above [`MAX_AGE`].
    AgeOutOfRange(u8),
    /// Height outside [`HEIGHT_RANGE_CM`].
    HeightOutOfRange(u16),
}

impl fmt::Display for ProfileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProfileError::AgeOutOfRange(age) => write!(
                f,
                "Age {age} is outside {MIN_AGE}-{MAX_AGE}. The meter only judges adults."
            ),
            ProfileError::HeightOutOfRange(h) => write!(
                f,
                "Height {h} cm is outside {}-{} cm. Are you a giraffe?",
                HEIGHT_RANGE_CM.0, HEIGHT_RANGE_CM.1
            ),
        }
    }
}

impl std::error::Error for ProfileError {}

/// Which input a contribution came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Factor {
    /// Age band.
    Age,
    /// Education, scaled by the regional norm factor.
    Education,
    /// Income band.
    Income,
    /// Height (only when given).
    Height,
    /// Self-rated looks.
    Looks,
    /// Self-rated build.
    Build,
    /// Self-rated intelligence.
    Smarts,
    /// Penis size (men and non-binary users who answer).
    Size,
    /// Relationship goal.
    Goal,
}

impl Factor {
    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Factor::Age => "Age",
            Factor::Education => "Education",
            Factor::Income => "Income",
            Factor::Height => "Height",
            Factor::Looks => "Looks",
            Factor::Build => "Build",
            Factor::Smarts => "Smarts",
            Factor::Size => "Size",
            Factor::Goal => "Goal",
        }
    }
}

/// One signed contribution to the score, with a short explanation.
#[derive(Debug, Clone, PartialEq)]
pub struct Contribution {
    /// Which input produced it.
    pub factor: Factor,
    /// Points added (positive) or removed (negative).
    pub delta: f32,
    /// One-line, tongue-in-cheek explanation.
    pub note: &'static str,
}

/// Headline tier, from best to worst.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Tier {
    /// Score 85 and up.
    TopOfTheMarket,
    /// Score 70 to below 85.
    HighlyCompetitive,
    /// Score 55 to below 70.
    SolidContender,
    /// Score 40 to below 55.
    NicheButLoved,
    /// Score 25 to below 40.
    ClearanceAisle,
    /// Score below 25. An obvious joke.
    LifeIsOver,
}

impl Tier {
    /// Every tier with its inclusive lower score bound, best first.
    pub const THRESHOLDS: [(Tier, f32); 6] = [
        (Tier::TopOfTheMarket, 85.0),
        (Tier::HighlyCompetitive, 70.0),
        (Tier::SolidContender, 55.0),
        (Tier::NicheButLoved, 40.0),
        (Tier::ClearanceAisle, 25.0),
        (Tier::LifeIsOver, f32::NEG_INFINITY),
    ];

    /// Tier for a score. Anything that is not at least 25 (including NaN)
    /// lands in [`Tier::LifeIsOver`].
    pub fn from_score(score: f32) -> Tier {
        Tier::THRESHOLDS
            .iter()
            .find(|(_, lower)| score >= *lower)
            .map_or(Tier::LifeIsOver, |(tier, _)| *tier)
    }

    /// Headline shown in big letters.
    pub const fn headline(self) -> &'static str {
        match self {
            Tier::TopOfTheMarket => "Top of the market",
            Tier::HighlyCompetitive => "Highly competitive asset",
            Tier::SolidContender => "Solid contender",
            Tier::NicheButLoved => "Niche, but with a loyal fan base",
            Tier::ClearanceAisle => "Clearance aisle (still a bargain)",
            Tier::LifeIsOver => "Your life is over (statistically, for this joke model) ;)",
        }
    }

    /// Short follow-up line under the headline.
    pub const fn tagline(self) -> &'static str {
        match self {
            Tier::TopOfTheMarket => "Analysts recommend: stay humble, it is a made-up index.",
            Tier::HighlyCompetitive => "Strong fundamentals. Try not to let it go to your head.",
            Tier::SolidContender => "Steady performer. Most great couples live right here.",
            Tier::NicheButLoved => "Not for everyone, which is exactly the point.",
            Tier::ClearanceAisle => "Undervalued by a model that cannot see your personality.",
            Tier::LifeIsOver => {
                "Good news: real people are not an additive model with a few dropdowns."
            }
        }
    }
}

/// The complete result of scoring a profile.
#[derive(Debug, Clone, PartialEq)]
pub struct Verdict {
    /// Final score, clamped to [`SCORE_RANGE`].
    pub score: f32,
    /// Headline tier for the score.
    pub tier: Tier,
    /// Gender of the scored profile (the lowest-tier meme depends on it).
    pub gender: Gender,
    /// Every contribution, largest absolute effect first.
    pub contributions: Vec<Contribution>,
}

/// Picks the woman or man weight, or their midpoint for non-binary users.
fn by_gender(gender: Gender, woman: f32, man: f32) -> f32 {
    match gender {
        Gender::Woman => woman,
        Gender::Man => man,
        Gender::NonBinary => (woman + man) / 2.0,
    }
}

/// Age contribution. Women's illustrative curve peaks earlier (25-29) than
/// men's (30-34), echoing the shape (not the size) of online-dating studies.
fn age_contribution(age: u8, gender: Gender) -> Contribution {
    let (woman, man, note) = match age {
        0..=24 => (10.0, -4.0, "Young: high demand, low savings."),
        25..=29 => (12.0, 4.0, "Late twenties: the model's favourite bracket."),
        30..=34 => (
            4.0,
            10.0,
            "Early thirties: peak 'has a sofa that is not a futon'.",
        ),
        35..=39 => (-6.0, 8.0, "Late thirties: the joke model gets judgy here."),
        40..=49 => (-14.0, 0.0, "Forties: the model ignores wisdom. Its loss."),
        _ => (
            -22.0,
            -10.0,
            "Fifty plus: the model was trained on app data, not on taste.",
        ),
    };
    Contribution {
        factor: Factor::Age,
        delta: by_gender(gender, woman, man),
        note,
    }
}

/// Education contribution, scaled by the regional norm factor.
///
/// For women the curve bends down at Master's and PhD: under reversed
/// educational hypergamy there are fewer same-or-higher educated men, so a
/// "marry up" market is tighter. For men more education is a mild plus.
fn education_contribution(education: Education, gender: Gender, region: Region) -> Contribution {
    let (woman, man, note) = match education {
        Education::HighSchool => (0.0, -6.0, "High school: the model is a snob, sorry."),
        Education::Bachelor => (4.0, 3.0, "Bachelor's: the sweet spot of the joke model."),
        Education::Master => (
            -2.0,
            6.0,
            "Master's: fewer 'equal or higher' partners if anyone insists on marrying up.",
        ),
        Education::Doctorate => (
            -8.0,
            6.0,
            "PhD: the tightest 'marry up' pool. Also, you talk about your thesis at dinner.",
        ),
    };
    Contribution {
        factor: Factor::Education,
        delta: by_gender(gender, woman, man) * region.education_norm_factor(),
        note,
    }
}

/// Income contribution. Larger for men than for women, but modest for both,
/// following the "stated sex difference is real but small" reading.
fn income_contribution(income: IncomeBand, gender: Gender) -> Contribution {
    let (woman, man, note) = match income {
        IncomeBand::Low => (-2.0, -10.0, "Low income: love is free, dinner is not."),
        IncomeBand::Middle => (0.0, 0.0, "Middle income: perfectly ordinary, no effect."),
        IncomeBand::UpperMiddle => (2.0, 6.0, "Upper middle: you can split the bill gracefully."),
        IncomeBand::High => (3.0, 10.0, "High income: the model is shallow and it shows."),
    };
    Contribution {
        factor: Factor::Income,
        delta: by_gender(gender, woman, man),
        note,
    }
}

/// Height contribution, purely for the joke. `None` when no height is given.
fn height_contribution(height_cm: Option<u16>, gender: Gender) -> Option<Contribution> {
    let height = height_cm?;
    let (woman, man, note) = match height {
        0..=169 => (
            0.0,
            -6.0,
            "Under 170 cm: the apps' height filter strikes again.",
        ),
        170..=179 => (1.0, 0.0, "170-179 cm: comfortably unremarkable."),
        180..=189 => (0.0, 5.0, "180-189 cm: the model is easily impressed."),
        _ => (-1.0, 7.0, "190 cm plus: mostly useful for top shelves."),
    };
    Some(Contribution {
        factor: Factor::Height,
        delta: by_gender(gender, woman, man),
        note,
    })
}

/// Looks contribution: the largest weight for both sexes, a little larger
/// for women (men's choices lean slightly more on looks).
fn looks_contribution(looks: Looks, gender: Gender) -> Contribution {
    let (woman, man, note) = match looks {
        Looks::BelowAverage => (
            -8.0,
            -7.0,
            "Below average: couples match on looks, so your match is out there squinting too.",
        ),
        Looks::Average => (
            0.0,
            0.0,
            "Average: statistically, where most of the love is.",
        ),
        Looks::AboveAverage => (7.0, 6.0, "Above average: the inbox has noticed."),
        Looks::HeadTurner => (
            11.0,
            10.0,
            "Head-turner: the model is shallow, and so are the apps.",
        ),
    };
    Contribution {
        factor: Factor::Looks,
        delta: by_gender(gender, woman, man),
        note,
    }
}

/// Build contribution: helps men more than women, and is capped.
fn build_contribution(build: Build, gender: Gender) -> Contribution {
    let (woman, man, note) = match build {
        Build::CouchPotato => (
            -1.0,
            -3.0,
            "Couch potato: the couch is very loyal, at least.",
        ),
        Build::Average => (0.0, 0.0, "Average: nobody has ever been dumped for this."),
        Build::GymRegular => (
            2.0,
            4.0,
            "Gym regular: shoulders noted. Please stop posting them.",
        ),
        Build::GreekStatue => (
            2.0,
            5.0,
            "Greek statue: capped. Statues are bad at texting back.",
        ),
    };
    Contribution {
        factor: Factor::Build,
        delta: by_gender(gender, woman, man),
        note,
    }
}

/// Smarts contribution: a modest plus that shrinks past the ceiling.
fn smarts_contribution(smarts: Smarts) -> Contribution {
    let (delta, note) = match smarts {
        Smarts::Average => (0.0, "Average: perfectly fine, nobody asks for an IQ test."),
        Smarts::Smart => (4.0, "Smart: one of the most wanted traits, for real."),
        Smarts::VerySmart => (
            6.0,
            "Very smart: still climbing, near the top of the curve.",
        ),
        Smarts::GalaxyBrain => (
            3.0,
            "Galaxy brain: past the ceiling. Please stop explaining things.",
        ),
    };
    Contribution {
        factor: Factor::Smarts,
        delta,
        note,
    }
}

/// Size contribution: a near-zero joke weight, men and non-binary users only.
/// `None` for women and for "prefer not to say".
fn size_contribution(size: Size, gender: Gender) -> Option<Contribution> {
    if gender == Gender::Woman {
        return None;
    }
    let (delta, note) = match size {
        Size::PreferNotToSay => return None,
        Size::BelowAverage => (
            -1.0,
            "Below average: most partners say they are satisfied. Relax.",
        ),
        Size::Average => (
            0.0,
            "Average: by definition, the usual. Nobody is counting.",
        ),
        Size::AboveAverage => (
            1.0,
            "Above average: one point. The model refuses to say more.",
        ),
        Size::Legendary => (
            1.0,
            "Legendary: also one point. Diminishing returns, and self-reported.",
        ),
    };
    Some(Contribution {
        factor: Factor::Size,
        delta,
        note,
    })
}

/// How much the relationship goal stretches or shrinks each factor.
/// Short-term choices lean on looks and build; long-term ones on status and
/// smarts (Kenrick et al. 1990; Li & Kenrick 2006). Goal itself is 1.
pub const fn goal_weight(goal: Goal, factor: Factor) -> f32 {
    match (goal, factor) {
        (Goal::Casual, Factor::Looks | Factor::Build) => 1.5,
        (Goal::Casual, Factor::Education | Factor::Income | Factor::Smarts) => 0.5,
        (Goal::LongTerm | Goal::Marriage, Factor::Education | Factor::Income | Factor::Smarts) => {
            1.2
        }
        _ => 1.0,
    }
}

/// Goal contribution: the market rewards having a plan.
fn goal_contribution(goal: Goal) -> Contribution {
    let (delta, note) = match goal {
        Goal::Casual => (3.0, "Casual: low commitment, high liquidity."),
        Goal::LongTerm => (2.0, "Long-term: a stable, dividend-paying strategy."),
        Goal::Marriage => (0.0, "Marriage: bold. The model respects it, quietly."),
        Goal::Undecided => (-2.0, "Undecided: the market hates uncertainty."),
    };
    Contribution {
        factor: Factor::Goal,
        delta,
        note,
    }
}

/// Checks the profile's free-form numbers.
pub fn validate(profile: &Profile) -> Result<(), ProfileError> {
    if !(MIN_AGE..=MAX_AGE).contains(&profile.age) {
        return Err(ProfileError::AgeOutOfRange(profile.age));
    }
    if let Some(h) = profile.height_cm
        && !(HEIGHT_RANGE_CM.0..=HEIGHT_RANGE_CM.1).contains(&h)
    {
        return Err(ProfileError::HeightOutOfRange(h));
    }
    Ok(())
}

/// Every contribution for a profile, in a fixed order (age, education,
/// income, height if given, goal). Does not validate.
pub fn contributions(profile: &Profile) -> Vec<Contribution> {
    let mut out = vec![
        age_contribution(profile.age, profile.gender),
        education_contribution(profile.education, profile.gender, profile.region),
        income_contribution(profile.income, profile.gender),
    ];
    out.extend(height_contribution(profile.height_cm, profile.gender));
    out.push(looks_contribution(profile.looks, profile.gender));
    out.push(build_contribution(profile.build, profile.gender));
    out.push(smarts_contribution(profile.smarts));
    out.extend(size_contribution(profile.size, profile.gender));
    out.push(goal_contribution(profile.goal));
    for c in &mut out {
        c.delta *= goal_weight(profile.goal, c.factor);
    }
    out
}

/// Base score plus contributions, clamped to [`SCORE_RANGE`].
pub fn total_score(contributions: &[Contribution]) -> f32 {
    let raw = BASE_SCORE + contributions.iter().map(|c| c.delta).sum::<f32>();
    raw.clamp(SCORE_RANGE.0, SCORE_RANGE.1)
}

/// Scores a profile: validates it, computes the contributions, the clamped
/// score and the tier. Contributions come back sorted by absolute effect.
pub fn evaluate(profile: &Profile) -> Result<Verdict, ProfileError> {
    validate(profile)?;
    let mut contributions = contributions(profile);
    let score = total_score(&contributions);
    contributions.sort_by(|a, b| b.delta.abs().total_cmp(&a.delta.abs()));
    Ok(Verdict {
        score,
        tier: Tier::from_score(score),
        gender: profile.gender,
        contributions,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn profile() -> Profile {
        Profile::default()
    }

    fn delta_of(p: &Profile, factor: Factor) -> Option<f32> {
        contributions(p)
            .into_iter()
            .find(|c| c.factor == factor)
            .map(|c| c.delta)
    }

    fn approx(a: f32, b: f32) -> bool {
        (a - b).abs() < 1e-4
    }

    #[test]
    fn tier_boundaries_are_inclusive_lower_bounds() {
        assert_eq!(Tier::from_score(100.0), Tier::TopOfTheMarket);
        assert_eq!(Tier::from_score(85.0), Tier::TopOfTheMarket);
        assert_eq!(Tier::from_score(84.99), Tier::HighlyCompetitive);
        assert_eq!(Tier::from_score(70.0), Tier::HighlyCompetitive);
        assert_eq!(Tier::from_score(69.99), Tier::SolidContender);
        assert_eq!(Tier::from_score(55.0), Tier::SolidContender);
        assert_eq!(Tier::from_score(54.99), Tier::NicheButLoved);
        assert_eq!(Tier::from_score(40.0), Tier::NicheButLoved);
        assert_eq!(Tier::from_score(39.99), Tier::ClearanceAisle);
        assert_eq!(Tier::from_score(25.0), Tier::ClearanceAisle);
        assert_eq!(Tier::from_score(24.99), Tier::LifeIsOver);
        assert_eq!(Tier::from_score(0.0), Tier::LifeIsOver);
        assert_eq!(Tier::from_score(f32::NAN), Tier::LifeIsOver);
    }

    #[test]
    fn thresholds_are_strictly_decreasing() {
        for pair in Tier::THRESHOLDS.windows(2) {
            assert!(pair[0].1 > pair[1].1);
            assert!(pair[0].0 < pair[1].0);
        }
    }

    #[test]
    fn default_profile_scores_as_expected() {
        // Woman, 28, Bachelor's, middle income, Turkey/ME, average looks,
        // build and smarts, long-term (education x1.2):
        // 60 + 12 + 4*1.3*1.2 + 0 + 0 + 0 + 0 + 2 = 80.24
        let v = evaluate(&profile()).expect("default profile is valid");
        assert!(approx(v.score, 80.24), "score was {}", v.score);
        assert_eq!(v.tier, Tier::HighlyCompetitive);
    }

    #[test]
    fn best_and_worst_profiles_reach_the_extreme_tiers() {
        let best = Profile {
            age: 32,
            gender: Gender::Man,
            education: Education::Doctorate,
            income: IncomeBand::High,
            height_cm: Some(192),
            looks: Looks::HeadTurner,
            region: Region::EastAsia,
            goal: Goal::Casual,
            ..Profile::default()
        };
        let worst = Profile {
            age: 60,
            gender: Gender::Woman,
            education: Education::Doctorate,
            income: IncomeBand::Low,
            height_cm: None,
            region: Region::EastAsia,
            goal: Goal::Undecided,
            ..Profile::default()
        };
        assert_eq!(evaluate(&best).map(|v| v.tier), Ok(Tier::TopOfTheMarket));
        assert_eq!(evaluate(&worst).map(|v| v.tier), Ok(Tier::LifeIsOver));
    }

    #[test]
    fn casual_goal_stretches_looks_and_shrinks_income() {
        let man = Profile {
            gender: Gender::Man,
            looks: Looks::HeadTurner,
            income: IncomeBand::High,
            ..profile()
        };
        let delta = |goal, factor| {
            contributions(&Profile { goal, ..man })
                .into_iter()
                .find(|c| c.factor == factor)
                .map(|c| c.delta)
        };
        assert_eq!(delta(Goal::Casual, Factor::Looks), Some(15.0));
        assert_eq!(delta(Goal::Undecided, Factor::Looks), Some(10.0));
        assert_eq!(delta(Goal::Casual, Factor::Income), Some(5.0));
        assert_eq!(delta(Goal::Marriage, Factor::Income), Some(12.0));
    }

    #[test]
    fn smarts_bonus_shrinks_past_the_ceiling() {
        let d = |smarts| smarts_contribution(smarts).delta;
        assert!(d(Smarts::VerySmart) > d(Smarts::GalaxyBrain));
        assert!(d(Smarts::GalaxyBrain) > d(Smarts::Average));
    }

    #[test]
    fn size_is_a_tiny_joke_and_ignored_for_women() {
        for size in Size::ALL {
            assert!(size_contribution(size, Gender::Woman).is_none());
            let delta = size_contribution(size, Gender::Man).map_or(0.0, |c| c.delta);
            assert!(delta.abs() <= 1.0);
        }
        assert!(size_contribution(Size::PreferNotToSay, Gender::Man).is_none());
    }

    #[test]
    fn score_is_clamped() {
        let huge = [Contribution {
            factor: Factor::Goal,
            delta: 1000.0,
            note: "",
        }];
        let tiny = [Contribution {
            factor: Factor::Goal,
            delta: -1000.0,
            note: "",
        }];
        assert_eq!(total_score(&huge), 100.0);
        assert_eq!(total_score(&tiny), 0.0);
        assert_eq!(total_score(&[]), BASE_SCORE);
    }

    #[test]
    fn non_binary_uses_midpoint_of_weights() {
        let base = Profile {
            age: 45,
            income: IncomeBand::High,
            ..profile()
        };
        for factor in [Factor::Age, Factor::Education, Factor::Income] {
            let w = delta_of(
                &Profile {
                    gender: Gender::Woman,
                    ..base
                },
                factor,
            );
            let m = delta_of(
                &Profile {
                    gender: Gender::Man,
                    ..base
                },
                factor,
            );
            let nb = delta_of(
                &Profile {
                    gender: Gender::NonBinary,
                    ..base
                },
                factor,
            );
            match (w, m, nb) {
                (Some(w), Some(m), Some(nb)) => assert!(approx(nb, (w + m) / 2.0)),
                other => panic!("missing contribution for {factor:?}: {other:?}"),
            }
        }
    }

    #[test]
    fn educated_women_market_is_tighter_where_marrying_up_norm_is_stronger() {
        let phd = |region| Profile {
            education: Education::Doctorate,
            region,
            ..profile()
        };
        let east = delta_of(&phd(Region::EastAsia), Factor::Education);
        let north = delta_of(&phd(Region::NorthernEurope), Factor::Education);
        match (east, north) {
            (Some(e), Some(n)) => {
                assert!(e < n);
                assert!(n < 0.0);
            }
            other => panic!("missing education contribution: {other:?}"),
        }
    }

    #[test]
    fn income_matters_more_for_men_but_is_monotone_for_both() {
        for gender in Gender::ALL {
            let deltas: Vec<f32> = IncomeBand::ALL
                .iter()
                .filter_map(|&income| {
                    delta_of(
                        &Profile {
                            gender,
                            income,
                            ..profile()
                        },
                        Factor::Income,
                    )
                })
                .collect();
            assert_eq!(deltas.len(), IncomeBand::ALL.len());
            assert!(
                deltas.windows(2).all(|w| w[0] <= w[1]),
                "{gender:?}: {deltas:?}"
            );
        }
        let spread = |gender| {
            let lo = delta_of(
                &Profile {
                    gender,
                    income: IncomeBand::Low,
                    ..profile()
                },
                Factor::Income,
            );
            let hi = delta_of(
                &Profile {
                    gender,
                    income: IncomeBand::High,
                    ..profile()
                },
                Factor::Income,
            );
            hi.zip(lo).map(|(h, l)| h - l)
        };
        assert!(spread(Gender::Man) > spread(Gender::Woman));
    }

    #[test]
    fn height_is_optional() {
        assert_eq!(delta_of(&profile(), Factor::Height), None);
        let tall = Profile {
            gender: Gender::Man,
            height_cm: Some(185),
            ..profile()
        };
        assert_eq!(delta_of(&tall, Factor::Height), Some(5.0));
    }

    #[test]
    fn invalid_inputs_are_errors_not_panics() {
        let young = Profile {
            age: 17,
            ..profile()
        };
        assert_eq!(evaluate(&young), Err(ProfileError::AgeOutOfRange(17)));
        let ancient = Profile {
            age: 121,
            ..profile()
        };
        assert_eq!(evaluate(&ancient), Err(ProfileError::AgeOutOfRange(121)));
        let giraffe = Profile {
            height_cm: Some(400),
            ..profile()
        };
        assert_eq!(evaluate(&giraffe), Err(ProfileError::HeightOutOfRange(400)));
    }

    #[test]
    fn contributions_sorted_by_absolute_effect() {
        let v = evaluate(&Profile {
            age: 45,
            ..profile()
        })
        .expect("valid profile");
        assert!(
            v.contributions
                .windows(2)
                .all(|w| w[0].delta.abs() >= w[1].delta.abs())
        );
    }

    /// Every combination of enum inputs crossed with a spread of ages and
    /// heights (about 34k profiles), built flat with iterators.
    fn all_profiles() -> impl Iterator<Item = Profile> {
        let ages = [18, 27, 33, 38, 45, 70, 120];
        let heights = [None, Some(160), Some(175), Some(185), Some(195)];
        let base = Profile::default();
        let gendered = Gender::ALL.into_iter().flat_map(move |gender| {
            Education::ALL.into_iter().map(move |education| Profile {
                gender,
                education,
                ..base
            })
        });
        let with_money = gendered.flat_map(|p| {
            IncomeBand::ALL
                .into_iter()
                .map(move |income| Profile { income, ..p })
        });
        let with_place = with_money.flat_map(|p| {
            Region::ALL
                .into_iter()
                .map(move |region| Profile { region, ..p })
        });
        let with_goal =
            with_place.flat_map(|p| Goal::ALL.into_iter().map(move |goal| Profile { goal, ..p }));
        let with_age = with_goal.flat_map(move |p| ages.map(|age| Profile { age, ..p }));
        with_age.flat_map(move |p| heights.map(|height_cm| Profile { height_cm, ..p }))
    }

    #[test]
    fn every_valid_profile_has_a_score_in_range() {
        for p in all_profiles() {
            let v = evaluate(&p).expect("generated profiles are valid");
            assert!((0.0..=100.0).contains(&v.score), "{p:?}");
            assert_eq!(v.tier, Tier::from_score(v.score));
        }
    }

    #[test]
    fn every_tier_is_reachable() {
        let reached: std::collections::HashSet<Tier> = all_profiles()
            .filter_map(|p| evaluate(&p).ok())
            .map(|v| v.tier)
            .collect();
        for (tier, _) in Tier::THRESHOLDS {
            assert!(reached.contains(&tier), "{tier:?} never reached");
        }
    }
}
