pub fn calculate_experience(level: u8) -> u32 {
    match level {
        1 => 0,
        2 => 500,
        3 => 1500,
        4 => 3750,
        5 => 7875,
        6 => 14175,
        7 => 22680,
        8 => 32886,
        9 => 44396,
        10 => 57715,
        11 => 72144,
        12 => 90180,
        13 => 112725,
        14 => 140906,
        15 => 176132,
        16 => 220165,
        17 => 275207,
        18 => 344008,
        19 => 430010,
        20 => 537513,
        21 => 671891,
        22 => 839864,
        23 => 1049830,
        24 => 1312287,
        25 => 1640359,
        26 => 2050449,
        27 => 2563061,
        28 => 3203826,
        29 => 3902260,
        30 => 4663553,
        31 => 5493363,
        32 => 6397855,
        33 => 7383752,
        34 => 8458379,
        35 => 9629723,
        36 => 10906488,
        37 => 12298162,
        38 => 13815086,
        39 => 15468534,
        40 => 17270791,
        41 => 19235252,
        42 => 21376515,
        43 => 23710491,
        44 => 26254525,
        45 => 29027522,
        46 => 32050088,
        47 => 35344686,
        48 => 38935798,
        49 => 42850109,
        50 => 47116709,
        51 => 51767302,
        52 => 56836449,
        53 => 62361819,
        54 => 68384473,
        55 => 74949165,
        56 => 82104680,
        57 => 89904191,
        58 => 98405658,
        59 => 107672256,
        60 => 117772849,
        61 => 128782495,
        62 => 140783010,
        63 => 153863570,
        64 => 168121381,
        65 => 183662396,
        66 => 200602101,
        67 => 219066380,
        68 => 239192444,
        69 => 261129853,
        70 => 285041630,
        71 => 311105466,
        72 => 339515048,
        73 => 370481492,
        74 => 404234916,
        75 => 441026148,
        76 => 481128591,
        77 => 524840254,
        78 => 572485967,
        79 => 624419793,
        80 => 681027665,
        81 => 742730244,
        82 => 809986056,
        83 => 883294891,
        84 => 963201521,
        85 => 1050299747,
        86 => 1145236814,
        87 => 1248718217,
        88 => 1361512946,
        89 => 1484459201,
        90 => 1618470619,
        91 => 1764543065,
        92 => 1923762030,
        93 => 2097310703,
        94 => 2286478756,
        95 => 2492671933,
        96 => 2717422497,
        97 => 2962400612,
        98 => 3229426756,
        99 => 3520485254,
        _ => panic!("Level cannot be above 99")
    }
}