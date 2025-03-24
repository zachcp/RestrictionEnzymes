//! # Restriction Enzyme Data
//!
//! The data in this package are derived from the [Biopython Restriction Dictionary](https://github.com/biopython/biopython/blob/master/Bio/Restriction/Restriction_Dictionary.py)
//! which in turn originate from [Rebase](http://rebase.neb.com).
//!
//! Those seeking to distribute REBASE files with their software packages are welcome to do so, providing it is clear to your users that they are not being charged for the REBASE data. It should be transparent that REBASE is a free and independent resource, with the following bibliographical reference:
//!  LATEST REVIEW: PDF file...
//!  Roberts, R.J., Vincze, T., Posfai, J., Macelis, D.
//!  REBASE-a database for DNA restriction and modification: enzymes, genes and genomes.
//!  Nucleic Acids Res. 43: D298-D299 (2015).
//!
//!
//!
//! name  pattern  len ncuts blunt c1 c2 c3 c4
//! Where:
//! name = name of enzyme
//! pattern = recognition site
//! len = length of pattern
//! ncuts = number of cuts made by enzyme
//!      Zero represents unknown
//! blunt = true if blunt end cut, false if sticky
//! c1 = First 5' cut
//! c2 = First 3' cut
//! c3 = Second 5' cut
//! c4 = Second 3' cut
//!
//! Examples:
//! AAC^TGG -> 6 2 1 3 3 0 0
//! A^ACTGG -> 6 2 0 1 5 0 0
//! AACTGG  -> 6 0 0 0 0 0 0
//! AACTGG(-5/-1) -> 6 2 0 1 5 0 0
//! (8/13)GACNNNNNNTCA(12/7) -> 12 4 0 -9 -14 24 19
//!
//! i.e. cuts are always to the right of the given
//! residue and sequences are always with reference to
//! the 5' strand.
//! Sequences are numbered ... -3 -2 -1 1 2 3 ... with
//! the first residue of the pattern at base number 1.
//!
//!
use enum_iterator::Sequence;
use include_dir::{Dir, include_dir};
use serde::{Deserialize, Serialize};
use serde_json;
use std::fmt::{self, Debug};

const DATA_DIR: Dir = include_dir!("data/restriction_enzymes/enzymedata");

/// https://github.com/rust-bio/rust-bio/blob/master/src/alphabets/dna.rs#L66
#[inline]
pub fn complement(base: char) -> char {
    match base {
        'a' => 't',
        'c' => 'g',
        't' => 'a',
        'g' => 'c',
        'n' => 't',
        'A' => 'T',
        'C' => 'G',
        'T' => 'A',
        'G' => 'C',
        'N' => 'N',
        'Y' => 'R',
        'R' => 'Y',
        'S' => 'S',
        'W' => 'W',
        'K' => 'M',
        'M' => 'K',
        'V' => 'B',
        'B' => 'V',
        'D' => 'H',
        'H' => 'D',
        // Todo
        _ => 'X',
    }
}

/// RestrictionEnzyme
/// Derived from Biopython which was drawn in turn from ReBase
#[derive(Debug, Serialize, Deserialize)]
pub struct RestrictionEnzyme {
    /// Enzyme Name, E.g. for Ecori:  "Ecori"
    pub name: String,
    /// E.g. for Ecori:  "(?=(?P<EcoRI>GAATTC))"
    compsite: String,
    /// Hmmm..... not sure.
    dna: Option<String>,
    /// Hmmm..... not sure.
    freq: f32,
    /// First 3' Cut
    fst3: Option<i32>,
    /// First 5' Cut
    fst5: Option<i32>,
    /// Rebase Identifier
    id: i32,
    /// Inactivation Temperature
    inact_temp: i32,
    /// Optimal Temperature
    opt_temp: i32,
    /// Overhang Size
    ovhg: Option<i32>,
    /// Overhang Sequence
    ovhgseq: Option<String>,
    /// Hmmm..... not sure.
    results: Option<String>,
    /// Second 3' Cut
    scd3: Option<i32>,
    /// Second 5' Cut
    scd5: Option<i32>,
    /// Genomic Site. The recognition pattern
    site: String,
    /// Size of the recognition site
    size: i32,
    /// Substrate. Usually "DNA"
    substrat: String,
    /// ## REBASE Supplier information for EMBOSS (embossre.sup)
    /// - B Thermo Fisher Scientific
    /// - C Minotech Biotechnology
    /// - E Agilent Technologies
    /// - I SibEnzyme Ltd.
    /// - J Nippon Gene Co., Ltd.
    /// - K Takara Bio Inc.
    /// - M Roche Applied Science
    /// - N New England Biolabs
    /// - O Toyobo Biochemicals
    /// - Q Molecular Biology Resources - CHIMERx
    /// - R Promega Corporation
    /// - S  Sigma Chemical Corporation
    /// - V Vivantis Technologies
    /// - X EURx Ltd.
    /// - Y SinaClon BioScience Co.
    ///
    pub suppl: Vec<String>,
    pub uri: String,
}

impl RestrictionEnzyme {
    pub fn reverse_complement_site(&self) -> String {
        self.site
            .chars()
            .rev()
            .map(|base| complement(base))
            .collect()
    }

    pub fn is_palindrome(&self) -> bool {
        &self.site == &self.reverse_complement_site()
    }

    pub fn is_end_sticky(&self) -> bool {
        &self.ovhgseq.clone().unwrap() != ""
    }

    pub fn is_end_blunt(&self) -> bool {
        &self.ovhgseq.clone().unwrap() == ""
    }
}

/// Enum of all of the Restriction Enzymes
/// Can be used to identify specific REs.
#[derive(Debug, Sequence)]
pub enum RestrictionEnzymeEnum {
    /// Aasi
    Aani,
    /// Aasi
    Aari,
    /// Aasi
    Aasi,
    Aatii,
    Aba13301i,
    Aba6411ii,
    Abab8342iv,
    Abaciii,
    Abapba3ii,
    Abasi,
    Abaumb2i,
    Abr4036ii,
    Absi,
    Acc16i,
    Acc36i,
    Acc65i,
    Acc65v,
    Accb1i,
    Accb7i,
    Accbsi,
    Acci,
    Accii,
    Acciii,
    Accix,
    Accx,
    Aceiii,
    Acha6iii,
    Acii,
    Acli,
    Aclwi,
    Aco12261ii,
    Acoi,
    Acoy31ii,
    Acsi,
    Acui,
    Acvi,
    Acyi,
    Adei,
    Adh6u21i,
    Afai,
    Afei,
    Afii,
    Aflii,
    Afliii,
    Agei,
    Agsi,
    Ahaiii,
    Ahdi,
    Ahli,
    Ahyrbahi,
    Ahyyl17i,
    Ajii,
    Ajni,
    Ajui,
    Alei,
    Alfi,
    Aloi,
    Alubi,
    Alui,
    Alw21i,
    Alw26i,
    Alw44i,
    Alwfi,
    Alwi,
    Alwni,
    Ama87i,
    Amacsi,
    Aod1i,
    Aor13hi,
    Aor51hi,
    Aoxi,
    Apabi,
    Apai,
    Apali,
    Apeki,
    Apoi,
    Apypi,
    Aquii,
    Aquiii,
    Aquiv,
    Arsi,
    Asci,
    Asei,
    Asi256i,
    Asigi,
    Asisi,
    Asl11923ii,
    Asp103i,
    Asp114pii,
    Asp337i,
    Asp700i,
    Asp718i,
    Aspa2i,
    Aspamdiv,
    Aspbhi,
    Aspdut2v,
    Aspjhl3ii,
    Asplei,
    Aspnih4iii,
    Asps9i,
    Aspslv7iii,
    Asu14238iv,
    Asuc2i,
    Asuhpi,
    Asui,
    Asuii,
    Asunhi,
    Ateti,
    Avai,
    Avaii,
    Avaiii,
    Avrii,
    Awo1030iv,
    Axyi,
    Baegi,
    Baei,
    Bag18758i,
    Bali,
    Bamhi,
    Bani,
    Banii,
    Banli,
    Bari,
    Bau1417v,
    Baui,
    Bbr52ii,
    Bbr57iii,
    Bbr7017ii,
    Bbr7017iii,
    Bbrpi,
    Bbsi,
    Bbub31i,
    Bbub31ii,
    Bbv12i,
    Bbvci,
    Bbvi,
    Bbvii,
    Bcci,
    Bce10661iii,
    Bce3081i,
    Bce83i,
    Bceai,
    Bcesiv,
    Bcefi,
    Bcgi,
    Bcit130i,
    Bcivi,
    Bcli,
    Bcni,
    Bco11035iii,
    Bcodi,
    Bcui,
    Bdai,
    Beti,
    Bfai,
    Bfasii,
    Bfii,
    Bfmi,
    Bfoi,
    Bfri,
    Bfuai,
    Bfui,
    Bga514i,
    Bgli,
    Bglii,
    Bini,
    Bisi,
    Bkram31di,
    Ble402ii,
    Blni,
    Bloaii,
    Blpi,
    Blsi,
    Bmcai,
    Bme1390i,
    Bme18i,
    Bmedi,
    Bmeri,
    Bmet110i,
    Bmgbi,
    Bmgi,
    Bmgt120i,
    Bmii,
    Bmrfi,
    Bmri,
    Bmsi,
    Bmti,
    Bmui,
    Boxi,
    Bpii,
    Bpli,
    Bpmi,
    Bps6700iii,
    Bpu10i,
    Bpu1102i,
    Bpu14i,
    Bpuei,
    Bpumi,
    Bsa29i,
    Bsaai,
    Bsabi,
    Bsahi,
    Bsai,
    Bsaji,
    Bsawi,
    Bsaxi,
    Bsbi,
    Bsc4i,
    Bscai,
    Bscgi,
    Bscxi,
    Bse118i,
    Bse1i,
    Bse21i,
    Bse3di,
    Bse8i,
    Bseai,
    Bsebi,
    Bseci,
    Bsedi,
    Bsegi,
    Bseji,
    Bseli,
    Bsemi,
    Bsemii,
    Bseni,
    Bsepi,
    Bseri,
    Bsesi,
    Bsex3i,
    Bsexi,
    Bseyi,
    Bsgi,
    Bsh1236i,
    Bsh1285i,
    Bshfi,
    Bshni,
    Bshti,
    Bshvi,
    Bsiei,
    Bsihkai,
    Bsihkci,
    Bsii,
    Bsisi,
    Bsiwi,
    Bsiyi,
    Bslfi,
    Bsli,
    Bsmai,
    Bsmbi,
    Bsmfi,
    Bsmi,
    Bsni,
    Bso31i,
    Bsobi,
    Bsp119i,
    Bsp120i,
    Bsp1286i,
    Bsp13i,
    Bsp1407i,
    Bsp143i,
    Bsp1720i,
    Bsp19i,
    Bsp24i,
    Bsp3004iv,
    Bsp460iii,
    Bsp68i,
    Bspaci,
    Bspani,
    Bspcni,
    Bspd6i,
    Bspdi,
    Bspei,
    Bspfni,
    Bspgi,
    Bsphi,
    Bspli,
    Bsplu11i,
    Bspmai,
    Bspmi,
    Bspmii,
    Bspnci,
    Bspoi,
    Bsppi,
    Bspqi,
    Bspt104i,
    Bspt107i,
    Bspti,
    Bsptni,
    Bsrbi,
    Bsrdi,
    Bsrfi,
    Bsrgi,
    Bsri,
    Bssai,
    Bsseci,
    Bsshii,
    Bssmi,
    Bssnai,
    Bssni,
    Bsssi,
    Bsst1i,
    Bst1107i,
    Bst2bi,
    Bst2ui,
    Bst4ci,
    Bst6i,
    Bstaci,
    Bstafi,
    Bstapi,
    Bstaui,
    Bstbai,
    Bstbi,
    Bstc8i,
    Bstdei,
    Bstdsi,
    Bsteii,
    Bsteni,
    Bstf5i,
    Bstfni,
    Bsth2i,
    Bsthhi,
    Bstkti,
    Bstmai,
    Bstmbi,
    Bstmci,
    Bstmwi,
    Bstni,
    Bstnsi,
    Bstpai,
    Bstpi,
    Bstsci,
    Bstsfi,
    Bstsli,
    Bstsni,
    Bstui,
    Bstv1i,
    Bstv2i,
    Bstx2i,
    Bstxi,
    Bstyi,
    Bstz17i,
    Bstzi,
    Bsu15i,
    Bsu36i,
    Bsui,
    Bsuri,
    Bsutui,
    Btgi,
    Btgzi,
    Bthci,
    Btri,
    Btsci,
    Btsi,
    Btsimuti,
    Btumi,
    Bve1b23i,
    Bvei,
    Cac8i,
    Caii,
    Cal14237i,
    Calb3ii,
    Cau10061ii,
    Cauii,
    Cba13ii,
    Cba16038i,
    Cbo67071iv,
    Cch467iii,
    Cchii,
    Cchiii,
    Ccii,
    Ccini,
    Cco11366vi,
    Cco11437v,
    Cco14983v,
    Cco14983vi,
    Ccrnaiii,
    Cdi11397i,
    Cdi13746v,
    Cdi13750iii,
    Cdii,
    Cdpi,
    Cdu23823ii,
    Cfa8380i,
    Cfoi,
    Cfr10i,
    Cfr13i,
    Cfr42i,
    Cfr9i,
    Cfri,
    Cfrmh13ii,
    Cfrmh16vi,
    Cfupf3ii,
    Cgl13032i,
    Cgl13032ii,
    Chai,
    Cin11811i,
    Cje265v,
    Cje54107iii,
    Cjefiii,
    Cjefv,
    Cjei,
    Cjenii,
    Cjeniii,
    Cjenv,
    Cjep659iv,
    Cjepi,
    Cjui,
    Cjuii,
    Cko11077iv,
    Cla11845iii,
    Clai,
    Cly7489ii,
    Cpe10578v,
    Cpe13170ii,
    Cpe2837iii,
    Cpoi,
    Cre7908i,
    Csa9238ii,
    Csei,
    Csii,
    Csp2014i,
    Csp6i,
    Cspai,
    Cspci,
    Cspi,
    Cstmi,
    Cviaii,
    Cviji,
    // Cviki1,
    Cviqi,
    Cviri,
    Dde51507i,
    Ddei,
    Dini,
    Dpi3069i,
    Dpi3084i,
    Dpi3090ii,
    Dpni,
    Dpnii,
    Drai,
    Draii,
    Draiii,
    Drari,
    Drdi,
    Drdii,
    Drdiv,
    Drdv,
    Drdvi,
    Drdviii,
    Drii,
    Dsai,
    Dsedi,
    Dsps02ii,
    Dvuiii,
    Eaei,
    Eagi,
    Eam1104i,
    Eam1105i,
    Eari,
    Ecii,
    Ecl136ii,
    Ecl234i,
    Ecl35734i,
    Eclxi,
    Eco105i,
    Eco130i,
    Eco147i,
    Eco24i,
    Eco31i,
    Eco32i,
    Eco4174i,
    Eco43896ii,
    Eco4465ii,
    Eco47i,
    Eco47iii,
    Eco52i,
    Eco53ki,
    Eco57i,
    Eco57mi,
    Eco72i,
    Eco8164i,
    Eco81i,
    Eco88i,
    Eco9009ii,
    Eco9020i,
    Eco9035i,
    Eco91i,
    Eco9699ii,
    Ecoblmcrx,
    Ecoe1140i,
    Ecohi,
    Ecohsi,
    Ecoicri,
    Ecomvii,
    Econi,
    Econih6ii,
    Ecoo109i,
    Ecoo65i,
    Ecori,
    Ecorii,
    Ecorv,
    Ecot14i,
    Ecot22i,
    Ecot38i,
    Egei,
    Ehei,
    Ehi46392i,
    Eli8509ii,
    Erhg4t10i,
    Erhi,
    Esabc3i,
    Esassi,
    Esp3007i,
    Esp3i,
    Espi,
    Faei,
    Faii,
    Fali,
    Faqi,
    Fati,
    Faui,
    Faundi,
    Fba202z8ii,
    Fbai,
    Fbli,
    Fco1691iv,
    Fini,
    Fmui,
    Fna13121i,
    Fnu11326ii,
    Fnu11326iv,
    Fnu4hi,
    Fnudii,
    Foki,
    Frioi,
    Fsei,
    Fsp4hi,
    Fspai,
    Fspbi,
    Fspei,
    Fspi,
    Fsppk15i,
    Ftnuv,
    Gaut27i,
    Gba708ii,
    Gdiii,
    Glai,
    Glui,
    Gru56503ii,
    Gsai,
    Gsui,
    Haei,
    Haeii,
    Haeiii,
    Hapii,
    Hauii,
    Hbaii,
    Hca13221v,
    Hdeny26i,
    Hdeza17i,
    Hgai,
    Hgiai,
    Hgici,
    Hgieii,
    Hgijii,
    Hhai,
    Hin1i,
    Hin1ii,
    Hin4i,
    Hin4ii,
    Hin6i,
    Hinp1i,
    Hincii,
    Hindii,
    Hindiii,
    Hinfi,
    Hpai,
    Hpaii,
    Hphi,
    Hpy166ii,
    Hpy178iii,
    Hpy188i,
    Hpy188iii,
    Hpy300xi,
    Hpy8i,
    Hpy99i,
    Hpy99xiii,
    Hpy99xiv,
    // Hpy99xivMut1,
    Hpy99xxii,
    Hpyas001vi,
    Hpyav,
    Hpyaxiv,
    Hpyaxviii,
    // HpyaxviMut1,
    // HpyaxviMut2,
    Hpych4iii,
    Hpych4iv,
    Hpych4v,
    Hpyf10vi,
    Hpyf3i,
    Hpylim6xii,
    Hpypu007xix,
    Hpyse526i,
    Hpyum032xiii,
    // Hpyum032xiiiMut1,
    Hpyum032xiv,
    Hpyum037x,
    Hso63250iv,
    Hso63373iii,
    Hsp92i,
    Hsp92ii,
    Hspai,
    Hspmhr1ii,
    Jma19592i,
    Jma19592ii,
    Jsp2502ii,
    Kas9737iii,
    Kasi,
    Kfli,
    Kor51ii,
    Kpn156v,
    Kpn2i,
    Kpn327i,
    Kpn9178i,
    Kpn9644ii,
    Kpni,
    Kpnnh25iii,
    Kpnnih30iii,
    Kpnnih50i,
    Kro7512ii,
    Kroi,
    Kroni,
    Ksp22i,
    Ksp632i,
    Kspai,
    Kspi,
    Kzo9i,
    Lba2029iii,
    Lbr124ii,
    Lde4408ii,
    Lgui,
    Llag50i,
    Lmni,
    Lmo370i,
    Lmo911ii,
    Lpl1004ii,
    Lpn11417ii,
    Lpn12272i,
    Lpni,
    Lpnpi,
    Lra68i,
    Lsads4i,
    Lsp1109i,
    Lsp48iii,
    Lsp6406vi,
    Lwei,
    Mabi,
    Maei,
    Maeii,
    Maeiii,
    Mali,
    Maqi,
    Maubi,
    Mba11i,
    Mbii,
    Mboi,
    Mboii,
    Mcati,
    Mch10819i,
    Mch946ii,
    Mcri,
    Mfei,
    Mfli,
    Mhli,
    Mjaiv,
    Mkadii,
    Mla10359i,
    Mlsi,
    Mlu211iii,
    Mluci,
    Mlui,
    Mluni,
    Mly113i,
    Mlyi,
    Mmei,
    Mnli,
    Mox20i,
    Mph1103i,
    Mrei,
    Mroi,
    Mroni,
    Mroxi,
    Msci,
    Msei,
    Msli,
    Msp20i,
    Mspa1i,
    Mspci,
    Mspf392i,
    Mspgi,
    Mspi,
    Mspi7ii,
    Mspi7iv,
    Mspji,
    Mspr9i,
    Mspsc27ii,
    Mssi,
    Msti,
    Mtei,
    Mtuhn878ii,
    Muni,
    Mva1269i,
    Mvai,
    Mvni,
    Mwoi,
    Naei,
    Nal45188ii,
    Nari,
    Nbr128ii,
    Ncii,
    Ncoi,
    Ndei,
    Ndeii,
    Ngoavii,
    Ngoaviii,
    Ngomiv,
    Nhaxi,
    Nhei,
    Nhoi,
    Nlaci,
    Nlaiii,
    Nlaiv,
    Nli3877i,
    Nmea6ciii,
    Nmeaiii,
    Nmedi,
    Nmuci,
    Noti,
    Npeus61ii,
    Nrui,
    Nsbi,
    Nsii,
    Nspbii,
    Nspes21ii,
    Nspi,
    Nspv,
    Obabs10i,
    Ogri,
    Olii,
    Osphl35iii,
    Pabi,
    Pac19842ii,
    Paci,
    Paciii,
    Pae10662iii,
    Pae8506i,
    Paei,
    Paepa99iii,
    Paer7i,
    Pagi,
    Pal408i,
    Palai,
    Paqci,
    Pasi,
    Paui,
    Pba2294i,
    Pbu13063ii,
    Pcaii,
    Pcei,
    Pcii,
    Pcisi,
    Pcr308ii,
    Pcsi,
    Pcti,
    Pdi8503iii,
    Pdii,
    Pdmi,
    Pdu1735i,
    Peni,
    Pfei,
    Pfl10783ii,
    Pfl1108i,
    Pfl23ii,
    Pfl3756ii,
    Pfl8569i,
    Pflfi,
    Pflmi,
    Pflpt14i,
    Pfoi,
    Pfrjs12iv,
    Pfrjs12v,
    Pfrjs15iii,
    Pin17fiii,
    Pinai,
    Pinp23ii,
    Pinp59iii,
    Pkri,
    Pladi,
    Ple19i,
    Plei,
    Plimi,
    Pluti,
    Pmaci,
    Pme10899i,
    Pmei,
    Pmli,
    Ppii,
    Ppip13ii,
    Ppsi,
    Ppu10i,
    Ppu21i,
    Ppumi,
    Pru8113i,
    Psci,
    Pse18267i,
    Pshai,
    Pshbi,
    Psii,
    Psp0357ii,
    Psp03i,
    Psp124bi,
    Psp1406i,
    Psp5ii,
    Psp6i,
    Pspci,
    Pspd7dii,
    Pspei,
    Pspfi,
    Pspgi,
    Pspli,
    Pspn4i,
    Pspomi,
    Pspomii,
    Psppi,
    Pspppi,
    Psppri,
    Pspr84i,
    Pspxi,
    Psri,
    Pssi,
    Pst14472i,
    Pst145i,
    Pst273i,
    Psti,
    Pstni,
    Psugi,
    Psui,
    Psyi,
    Ptei,
    Pvui,
    Pvuii,
    Ran11014iv,
    Rba2021i,
    Rcei,
    Rdegbi,
    Rdegbii,
    Rdegbiii,
    Rer8036ii,
    Rflfiii,
    Rgai,
    Rgo13296iv,
    Rho5650i,
    Rigi,
    Rkr11038i,
    Rlai,
    Rlaii,
    Rleai,
    Rmu369iii,
    Rpab5i,
    Rpabi,
    Rpai,
    Rpati,
    Rrui,
    Rsai,
    Rsani,
    Rsei,
    Rsp008iv,
    Rsp008v,
    Rsp531ii,
    Rsppbts2iii,
    Rsr2i,
    Rsrii,
    Rtr1953i,
    Saci,
    Sacii,
    Saf8902iii,
    Sag901i,
    Sali,
    Sandi,
    Sapi,
    Saqai,
    Sati,
    Sau1803iii,
    Sau3ai,
    Sau5656ii,
    Sau64037iv,
    Sau96i,
    Saui,
    Saumj015iii,
    Sba460ii,
    Sbfi,
    Sbo46i,
    Scai,
    Schi,
    Scii,
    Scods2ii,
    Scrfi,
    Sdai,
    Sdeai,
    Sdeosi,
    Sdui,
    Sdy5370i,
    Sdy7136i,
    Sdy9603i,
    Seci,
    Seli,
    Sen17963iii,
    Sen5794iii,
    Sen6480iv,
    Sena1673iii,
    Sensara26iii,
    Sentfiv,
    Sep11964i,
    Seti,
    Sexai,
    Sfaai,
    Sfani,
    Sfci,
    Sfei,
    Sfii,
    Sfl13829iii,
    Sfoi,
    Sfr274i,
    Sfr303i,
    Sfui,
    Sgei,
    Sgfi,
    Sgr7807i,
    Sgrai,
    Sgraii,
    Sgrbi,
    Sgrdi,
    Sgrti,
    Sgsi,
    Simi,
    Sini,
    Slai,
    Sma10259ii,
    Sma325i,
    Smai,
    Smaumh5i,
    Smaumh8i,
    Smii,
    Smimi,
    Smli,
    Smoi,
    Sna507viii,
    Snabi,
    Snai,
    Sno506i,
    Spe19205iv,
    Spei,
    Sphi,
    Spli,
    Spnrii,
    Spodi,
    Srfi,
    Sse232i,
    Sse8387i,
    Sse8647i,
    Sse9i,
    Ssebi,
    Ssii,
    Ssp6803iv,
    Ssp714ii,
    Sspd5i,
    Sspdi,
    Sspi,
    Sspjor1ii,
    Sspmi,
    Sste37i,
    Ssti,
    Sth132i,
    Sth20745iii,
    Sth302ii,
    Sthst3ii,
    Stsi,
    Stui,
    Styd4i,
    Styi,
    Surp32aii,
    Swai,
    Taai,
    Tagi,
    Taii,
    Taqi,
    Taqii,
    Taqiii,
    Tasi,
    Tati,
    Taui,
    Tfii,
    Tkoi,
    Tkoii,
    Tpytp2i,
    Tru1i,
    Tru9i,
    Tscai,
    Tsefi,
    Tsei,
    Tsoi,
    Tsp45i,
    Tsp4ci,
    Tsparh3i,
    Tspdti,
    Tspei,
    Tspgwi,
    Tspmi,
    Tspri,
    Tssi,
    Tsti,
    Tsui,
    Tth111i,
    Tth111ii,
    Ubaf11i,
    Ubaf12i,
    Ubaf13i,
    Ubaf14i,
    Ubaf9i,
    Ubapi,
    Ucomsi,
    Unbi,
    Van9116i,
    Van91i,
    Vche4ii,
    Vdi96ii,
    Vha464i,
    Vnei,
    Vpak11ai,
    Vpak11bi,
    Vspi,
    Vtu19109i,
    Wvii,
    Xagi,
    Xapi,
    Xbai,
    Xca85iv,
    Xcei,
    Xcmi,
    Xhoi,
    Xhoii,
    Xmai,
    Xmaiii,
    Xmaji,
    Xmii,
    Xmni,
    Xspi,
    Ykri,
    Yps3606i,
    Yru12986i,
    Zrai,
    Zrmi,
    Zsp2i,
}

#[cfg(test)]
mod tests {
    use super::*;
    use enum_iterator::{all, cardinality};
    #[test]
    fn test_cardinality() {
        // I've commentsed out 5  JSON files that do not have an ID
        // assert_eq!(cardinality::<RestrictionEnzymeEnum>(), 1065);
        assert_eq!(cardinality::<RestrictionEnzymeEnum>(), 1060);
    }

    #[test]
    fn test_can_load() {
        for e in all::<RestrictionEnzymeEnum>() {
            println!("{:?}", e);
            let res = e.value();
            println!("{:?}", res)
            // res
        }
    }

    #[test]
    fn test_palindromic() {
        let mut palindromic: Vec<bool> = vec![];
        for e in all::<RestrictionEnzymeEnum>() {
            let res = e.value();
            palindromic.push(res.is_palindrome())
        }
        assert_eq!(palindromic.len(), 1060)
    }

    #[test]
    fn test_re_data_loaded() {
        let aari = RestrictionEnzymeEnum::Aari.value();
        println!("aari! {:?}", aari);
        assert_eq!(aari.name, "Aari");
        assert_eq!(
            aari.compsite,
            "(?=(?P<AarI>CACCTGC))|(?=(?P<AarI_as>GCAGGTG))"
        );
        assert_eq!(aari.dna, None);
        assert_eq!(aari.freq, 16384.0);
        assert_eq!(aari.fst3.unwrap(), 8);
        assert_eq!(aari.fst5.unwrap(), 11);
        assert_eq!(aari.id, 2892);
        assert_eq!(aari.inact_temp, 65);
        assert_eq!(aari.opt_temp, 37);
        assert_eq!(aari.ovhg.unwrap(), -4);
        assert_eq!(aari.ovhgseq.unwrap(), "NNNN");
        assert_eq!(aari.results, None);
        assert_eq!(aari.scd3, None);
        assert_eq!(aari.scd5, None);
        assert_eq!(aari.site, "CACCTGC");
        assert_eq!(aari.size, 7);
        assert_eq!(aari.substrat, "DNA");
        assert_eq!(aari.suppl, ["B"]);
        assert_eq!(aari.uri, "https://identifiers.org/rebase:2892");
    }

    #[test]
    fn test_re_data_loaded_2() {
        let zsp2i = RestrictionEnzymeEnum::Zsp2i.value();
        println!("zsp2i! {:?}", zsp2i);
        assert_eq!(zsp2i.name, "Zsp2i");
        assert_eq!(zsp2i.compsite, "(?=(?P<Zsp2I>ATGCAT))");
        assert_eq!(zsp2i.dna, None);
        assert_eq!(zsp2i.freq, 4096.0);
        assert_eq!(zsp2i.fst3.unwrap(), -5);
        assert_eq!(zsp2i.fst5.unwrap(), 5);
        assert_eq!(zsp2i.id, 2156);
        assert_eq!(zsp2i.inact_temp, 65);
        assert_eq!(zsp2i.opt_temp, 37);
        assert_eq!(zsp2i.ovhg.unwrap(), 4);
        assert_eq!(zsp2i.ovhgseq.unwrap(), "TGCA");
        assert_eq!(zsp2i.results, None);
        assert_eq!(zsp2i.scd3, None);
        assert_eq!(zsp2i.scd5, None);
        assert_eq!(zsp2i.site, "ATGCAT");
        assert_eq!(zsp2i.size, 6);
        assert_eq!(zsp2i.substrat, "DNA");
        assert_eq!(zsp2i.suppl, ["I", "V"]);
        assert_eq!(zsp2i.uri, "https://identifiers.org/rebase:2156");
    }

    #[test]
    fn test_restriction_enzyme_fns() {
        let zsp2i = RestrictionEnzymeEnum::Zsp2i.value();
        assert_eq!(zsp2i.site, "ATGCAT");
        assert_eq!(zsp2i.site, zsp2i.reverse_complement_site());

        let ecori = RestrictionEnzymeEnum::Ecori.value();
        assert_eq!(ecori.site, "GAATTC");
        assert_eq!(ecori.site, ecori.reverse_complement_site());

        let smai = RestrictionEnzymeEnum::Smai.value();
        assert!(smai.is_end_blunt());
        assert!(!smai.is_end_sticky());
    }
}
