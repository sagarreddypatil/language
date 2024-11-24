function id(x) {
    return x
}

function data(d, ...fields) {
    return [ d, fields ]
}

function desc(data) {
    return data[0]
}

function field(data, i) {
    return data[1][i]
}

function halt(x) {
    const util = require('util')
    console.log(util.inspect(x, { depth: null }))
}

function sorted(rc_1, list) {
    function match_after_1(matched_1) {
        rc_1(matched_1)
    }
    function pm_alt_1() {
        function pm_alt_2() {
            const d1_2 = 1;
            let desc_2 = desc(list);
            function pm_good_3() {
                const i0_1 = 0;
                let f0_1 = field(list, i0_1);
                let f1_1 = field(list, d1_2);
                let x = id(f0_1);
                let desc_1 = desc(f1_1);
                function pm_good_4() {
                    let f0_2 = field(f1_1, i0_1);
                    let f1_2 = field(f1_1, d1_2);
                    let y = id(f0_2);
                    let rest = id(f1_2);
                    let prim_1 = x <= y;
                    function match_after_2(matched_2) {
                        match_after_1(matched_2)
                    }
                    function pm_alt_3() {
                        function pm_good_1() {
                            match_after_2(i0_1)
                        }
                        if (i0_1 == prim_1) { pm_good_1() } else { halt() }
                    }
                    function pm_good_2() {
                        function rc_2(rv_1) {
                            match_after_2(rv_1)
                        }
                        let data_Cons_1 = data(d1_2, y, rest);
                        sorted(rc_2, data_Cons_1)
                    }
                    if (d1_2 == prim_1) { pm_good_2() } else { pm_alt_3() }
                }
                if (d1_2 == desc_1) { pm_good_4() } else { halt() }
            }
            if (d1_2 == desc_2) { pm_good_3() } else { halt() }
        }
        const d1_4 = 1;
        let desc_4 = desc(list);
        function pm_good_5() {
            const i0_3 = 0;
            let f0_3 = field(list, i0_3);
            let f1_3 = field(list, d1_4);
            let _ = id(f0_3);
            let desc_3 = desc(f1_3);
            function pm_good_6() {
                match_after_1(d1_4)
            }
            if (i0_3 == desc_3) { pm_good_6() } else { pm_alt_2() }
        }
        if (d1_4 == desc_4) { pm_good_5() } else { pm_alt_2() }
    }
    const d0_2 = 0;
    let desc_5 = desc(list);
    function pm_good_7() {
        const c1_2 = 1;
        match_after_1(c1_2)
    }
    if (d0_2 == desc_5) { pm_good_7() } else { pm_alt_1() }
}
function bubble(rc_3, list) {
    function match_after_3(matched_3) {
        rc_3(matched_3)
    }
    function pm_alt_4() {
        function pm_alt_5() {
            const d1_9 = 1;
            let desc_7 = desc(list);
            function pm_good_10() {
                const i0_4 = 0;
                let f0_4 = field(list, i0_4);
                let f1_4 = field(list, d1_9);
                let x = id(f0_4);
                let desc_6 = desc(f1_4);
                function pm_good_11() {
                    let f0_5 = field(f1_4, i0_4);
                    let f1_5 = field(f1_4, d1_9);
                    let y = id(f0_5);
                    let rest = id(f1_5);
                    let prim_2 = x <= y;
                    function match_after_4(matched_4) {
                        match_after_3(matched_4)
                    }
                    function pm_alt_6() {
                        function pm_good_8() {
                            function rc_4(rv_2) {
                                let data_Cons_2 = data(d1_9, y, rv_2);
                                match_after_4(data_Cons_2)
                            }
                            let data_Cons_3 = data(d1_9, x, rest);
                            bubble(rc_4, data_Cons_3)
                        }
                        if (i0_4 == prim_2) { pm_good_8() } else { halt() }
                    }
                    function pm_good_9() {
                        function rc_5(rv_3) {
                            let data_Cons_4 = data(d1_9, x, rv_3);
                            match_after_4(data_Cons_4)
                        }
                        let data_Cons_5 = data(d1_9, y, rest);
                        bubble(rc_5, data_Cons_5)
                    }
                    if (d1_9 == prim_2) { pm_good_9() } else { pm_alt_6() }
                }
                if (d1_9 == desc_6) { pm_good_11() } else { halt() }
            }
            if (d1_9 == desc_7) { pm_good_10() } else { halt() }
        }
        const d1_12 = 1;
        let desc_9 = desc(list);
        function pm_good_12() {
            const i0_6 = 0;
            let f0_6 = field(list, i0_6);
            let f1_6 = field(list, d1_12);
            let x = id(f0_6);
            let desc_8 = desc(f1_6);
            function pm_good_13() {
                let data_Nil_1 = data(i0_6);
                let data_Cons_6 = data(d1_12, x, data_Nil_1);
                match_after_3(data_Cons_6)
            }
            if (i0_6 == desc_8) { pm_good_13() } else { pm_alt_5() }
        }
        if (d1_12 == desc_9) { pm_good_12() } else { pm_alt_5() }
    }
    const d0_6 = 0;
    let desc_10 = desc(list);
    function pm_good_14() {
        let data_Nil_2 = data(d0_6);
        match_after_3(data_Nil_2)
    }
    if (d0_6 == desc_10) { pm_good_14() } else { pm_alt_4() }
}
function sort(rc_6, list) {
    function rc_7(rv_4) {
        function match_after_5(matched_5) {
            rc_6(matched_5)
        }
        function pm_alt_7() {
            const p0_3 = 0;
            function pm_good_15() {
                function rc_8(rv_5) {
                    match_after_5(rv_5)
                }
                function rc_9(rv_6) {
                    sort(rc_8, rv_6)
                }
                bubble(rc_9, list)
            }
            if (p0_3 == rv_4) { pm_good_15() } else { halt() }
        }
        const p1_3 = 1;
        function pm_good_16() {
            match_after_5(list)
        }
        if (p1_3 == rv_4) { pm_good_16() } else { pm_alt_7() }
    }
    sorted(rc_7, list)
}
function append(rc_10, list1, list2) {
    function match_after_6(matched_6) {
        rc_10(matched_6)
    }
    function pm_alt_8() {
        const d1_14 = 1;
        let desc_11 = desc(list1);
        function pm_good_17() {
            const i0_7 = 0;
            let f0_7 = field(list1, i0_7);
            let f1_7 = field(list1, d1_14);
            let x = id(f0_7);
            let rest = id(f1_7);
            function rc_11(rv_7) {
                let data_Cons_7 = data(d1_14, x, rv_7);
                match_after_6(data_Cons_7)
            }
            append(rc_11, rest, list2)
        }
        if (d1_14 == desc_11) { pm_good_17() } else { halt() }
    }
    const d0_7 = 0;
    let desc_12 = desc(list1);
    function pm_good_18() {
        match_after_6(list2)
    }
    if (d0_7 == desc_12) { pm_good_18() } else { pm_alt_8() }
}
function reverse(rc_12, list) {
    function match_after_7(matched_7) {
        rc_12(matched_7)
    }
    function pm_alt_9() {
        const d1_16 = 1;
        let desc_13 = desc(list);
        function pm_good_19() {
            const i0_8 = 0;
            let f0_8 = field(list, i0_8);
            let f1_8 = field(list, d1_16);
            let x = id(f0_8);
            let rest = id(f1_8);
            function rc_13(rv_8) {
                match_after_7(rv_8)
            }
            function rc_14(rv_9) {
                let data_Nil_3 = data(i0_8);
                let data_Cons_8 = data(d1_16, x, data_Nil_3);
                append(rc_13, rv_9, data_Cons_8)
            }
            reverse(rc_14, rest)
        }
        if (d1_16 == desc_13) { pm_good_19() } else { halt() }
    }
    const d0_10 = 0;
    let desc_14 = desc(list);
    function pm_good_20() {
        let data_Nil_4 = data(d0_10);
        match_after_7(data_Nil_4)
    }
    if (d0_10 == desc_14) { pm_good_20() } else { pm_alt_9() }
}
const c4_1 = 4;
const c5_1 = 5;
const c1_3 = 1;
const c2_1 = 2;
const c3_1 = 3;
const d0_11 = 0;
let data_Nil_5 = data(d0_11);
let data_Cons_13 = data(c1_3, c3_1, data_Nil_5);
let data_Cons_12 = data(c1_3, c2_1, data_Cons_13);
let data_Cons_11 = data(c1_3, c1_3, data_Cons_12);
let data_Cons_10 = data(c1_3, c5_1, data_Cons_11);
let data_Cons_9 = data(c1_3, c4_1, data_Cons_10);
let list = id(data_Cons_9);
function rc_15(rv_10) {
    halt(rv_10)
}
function rc_16(rv_11) {
    reverse(rc_15, rv_11)
}
sort(rc_16, list)