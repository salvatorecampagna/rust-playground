use raindrops_plung::*;

#[test]
fn the_sound_for_1_is_1() {
    let input = 1;
    let output = raindrops(input);
    let expected = "1";
    assert_eq!(output, expected);
}

#[test]
fn the_sound_for_3_is_pling() {
    let input = 3;
    let output = raindrops(input);
    let expected = "Pling";
    assert_eq!(output, expected);
}

#[test]
fn the_sound_for_5_is_plang() {
    let input = 5;
    let output = raindrops(input);
    let expected = "Plang";
    assert_eq!(output, expected);
}

#[test]
fn the_sound_for_7_is_plong() {
    let input = 7;
    let output = raindrops(input);
    let expected = "Plong";
    assert_eq!(output, expected);
}

#[test]
fn the_sound_for_11_is_plung() {
    let input = 11;
    let output = raindrops(input);
    let expected = "Plung";
    assert_eq!(output, expected);
}

#[test]
fn the_sound_for_22_is_plung_as_it_has_a_factor_11() {
    let input = 22;
    let output = raindrops(input);
    let expected = "Plung";
    assert_eq!(output, expected);
}

#[test]
fn the_sound_for_121_is_plung_as_it_has_a_factor_11() {
    let input = 121;
    let output = raindrops(input);
    let expected = "Plung";
    assert_eq!(output, expected);
}

#[test]
fn the_sound_for_13_is_13() {
    let input = 13;
    let output = raindrops(input);
    let expected = "13";
    assert_eq!(output, expected);
}

#[test]
fn the_sound_for_15_is_plingplang() {
    let input = 15;
    let output = raindrops(input);
    let expected = "PlingPlang";
    assert_eq!(output, expected);
}

#[test]
fn the_sound_for_21_is_plingplong() {
    let input = 21;
    let output = raindrops(input);
    let expected = "PlingPlong";
    assert_eq!(output, expected);
}

#[test]
fn the_sound_for_33_is_plingplung() {
    let input = 33;
    let output = raindrops(input);
    let expected = "PlingPlung";
    assert_eq!(output, expected);
}

#[test]
fn the_sound_for_35_is_plangplong() {
    let input = 35;
    let output = raindrops(input);
    let expected = "PlangPlong";
    assert_eq!(output, expected);
}

#[test]
fn the_sound_for_55_is_plangplung() {
    let input = 55;
    let output = raindrops(input);
    let expected = "PlangPlung";
    assert_eq!(output, expected);
}

#[test]
fn the_sound_for_77_is_plongplung() {
    let input = 77;
    let output = raindrops(input);
    let expected = "PlongPlung";
    assert_eq!(output, expected);
}

#[test]
fn the_sound_for_105_is_plingplangplong() {
    let input = 105;
    let output = raindrops(input);
    let expected = "PlingPlangPlong";
    assert_eq!(output, expected);
}

#[test]
fn the_sound_for_165_is_plingplangplung() {
    let input = 165;
    let output = raindrops(input);
    let expected = "PlingPlangPlung";
    assert_eq!(output, expected);
}

#[test]
fn the_sound_for_231_is_plingplongplung() {
    let input = 231;
    let output = raindrops(input);
    let expected = "PlingPlongPlung";
    assert_eq!(output, expected);
}

#[test]
fn the_sound_for_385_is_plangplongplung() {
    let input = 385;
    let output = raindrops(input);
    let expected = "PlangPlongPlung";
    assert_eq!(output, expected);
}

#[test]
fn the_sound_for_1155_is_plingplangplongplung() {
    let input = 1155;
    let output = raindrops(input);
    let expected = "PlingPlangPlongPlung";
    assert_eq!(output, expected);
}

#[test]
fn the_sound_for_169_is_169_as_13_squared_is_not_a_raindrop() {
    let input = 169;
    let output = raindrops(input);
    let expected = "169";
    assert_eq!(output, expected);
}

#[test]
fn the_sound_for_0_is_every_sound() {
    let input = 0;
    let output = raindrops(input);
    let expected = "PlingPlangPlongPlung";
    assert_eq!(output, expected);
}

#[test]
fn the_sound_for_the_largest_u32_is_plingplang() {
    // u32::MAX = 4294967295 = 3 * 5 * 17 * 257 * 65537
    let input = u32::MAX;
    let output = raindrops(input);
    let expected = "PlingPlang";
    assert_eq!(output, expected);
}
