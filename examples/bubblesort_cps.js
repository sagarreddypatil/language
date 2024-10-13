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

function fn_1(rc_1, list) {
    function match_after_1(matched_1) {
        rc_1(matched_1)
    }
    function m_alt_1() {
        function m_alt_2() {
            const d1 = 1;
            let desc_2 = desc(list);
            function pm_good_3() {
                const i0 = 0;
                let f0_1 = field(list, i0);
                const i1 = 1;
                let f1_1 = field(list, i1);
                let x = id(f0_1);
                const d1 = 1;
                let desc_1 = desc(f1_1);
                function pm_good_4() {
                    const i0 = 0;
                    let f0_2 = field(f1_1, i0);
                    const i1 = 1;
                    let f1_2 = field(f1_1, i1);
                    let y = id(f0_2);
                    let rest = id(f1_2);
                    let prim_1 = x <= y;
                    function match_after_2(matched_2) {
                        match_after_1(matched_2)
                    }
                    function m_alt_3() {
                        const p0 = 0;
                        function pm_good_1() {
                            const c0 = 0;
                            match_after_2(c0)
                        }
                        if (p0 == prim_1) { pm_good_1() } else { halt() }
                    }
                    const p1 = 1;
                    function pm_good_2() {
                        function rc_2(rv_1) {
                            match_after_2(rv_1)
                        }
                        const d1 = 1;
                        let data_Cons_1 = data(d1, y, rest);
                        sorted(rc_2, data_Cons_1)
                    }
                    if (p1 == prim_1) { pm_good_2() } else { m_alt_3() }
                }
                if (d1 == desc_1) { pm_good_4() } else { halt() }
            }
            if (d1 == desc_2) { pm_good_3() } else { halt() }
        }
        const d1 = 1;
        let desc_4 = desc(list);
        function pm_good_5() {
            const i0 = 0;
            let f0_3 = field(list, i0);
            const i1 = 1;
            let f1_3 = field(list, i1);
            let _ = id(f0_3);
            const d0 = 0;
            let desc_3 = desc(f1_3);
            function pm_good_6() {
                const c1 = 1;
                match_after_1(c1)
            }
            if (d0 == desc_3) { pm_good_6() } else { m_alt_2() }
        }
        if (d1 == desc_4) { pm_good_5() } else { m_alt_2() }
    }
    const d0 = 0;
    let desc_5 = desc(list);
    function pm_good_7() {
        const c1 = 1;
        match_after_1(c1)
    }
    if (d0 == desc_5) { pm_good_7() } else { m_alt_1() }
}
let sorted = id(fn_1);
function fn_2(rc_3, list) {
    function match_after_3(matched_3) {
        rc_3(matched_3)
    }
    function m_alt_4() {
        function m_alt_5() {
            const d1 = 1;
            let desc_7 = desc(list);
            function pm_good_10() {
                const i0 = 0;
                let f0_4 = field(list, i0);
                const i1 = 1;
                let f1_4 = field(list, i1);
                let x = id(f0_4);
                const d1 = 1;
                let desc_6 = desc(f1_4);
                function pm_good_11() {
                    const i0 = 0;
                    let f0_5 = field(f1_4, i0);
                    const i1 = 1;
                    let f1_5 = field(f1_4, i1);
                    let y = id(f0_5);
                    let rest = id(f1_5);
                    let prim_2 = x <= y;
                    function match_after_4(matched_4) {
                        match_after_3(matched_4)
                    }
                    function m_alt_6() {
                        const p0 = 0;
                        function pm_good_8() {
                            function rc_4(rv_2) {
                                const d1 = 1;
                                let data_Cons_2 = data(d1, y, rv_2);
                                match_after_4(data_Cons_2)
                            }
                            const d1 = 1;
                            let data_Cons_3 = data(d1, x, rest);
                            bubble(rc_4, data_Cons_3)
                        }
                        if (p0 == prim_2) { pm_good_8() } else { halt() }
                    }
                    const p1 = 1;
                    function pm_good_9() {
                        function rc_5(rv_3) {
                            const d1 = 1;
                            let data_Cons_4 = data(d1, x, rv_3);
                            match_after_4(data_Cons_4)
                        }
                        const d1 = 1;
                        let data_Cons_5 = data(d1, y, rest);
                        bubble(rc_5, data_Cons_5)
                    }
                    if (p1 == prim_2) { pm_good_9() } else { m_alt_6() }
                }
                if (d1 == desc_6) { pm_good_11() } else { halt() }
            }
            if (d1 == desc_7) { pm_good_10() } else { halt() }
        }
        const d1 = 1;
        let desc_9 = desc(list);
        function pm_good_12() {
            const i0 = 0;
            let f0_6 = field(list, i0);
            const i1 = 1;
            let f1_6 = field(list, i1);
            let x = id(f0_6);
            const d0 = 0;
            let desc_8 = desc(f1_6);
            function pm_good_13() {
                const d0 = 0;
                let data_Nil_1 = data(d0);
                const d1 = 1;
                let data_Cons_6 = data(d1, x, data_Nil_1);
                match_after_3(data_Cons_6)
            }
            if (d0 == desc_8) { pm_good_13() } else { m_alt_5() }
        }
        if (d1 == desc_9) { pm_good_12() } else { m_alt_5() }
    }
    const d0 = 0;
    let desc_10 = desc(list);
    function pm_good_14() {
        const d0 = 0;
        let data_Nil_2 = data(d0);
        match_after_3(data_Nil_2)
    }
    if (d0 == desc_10) { pm_good_14() } else { m_alt_4() }
}
let bubble = id(fn_2);
function fn_3(rc_6, list) {
    function rc_7(rv_4) {
        function match_after_5(matched_5) {
            rc_6(matched_5)
        }
        function m_alt_7() {
            const p0 = 0;
            function pm_good_15() {
                function rc_8(rv_5) {
                    match_after_5(rv_5)
                }
                function rc_9(rv_6) {
                    sort(rc_8, rv_6)
                }
                bubble(rc_9, list)
            }
            if (p0 == rv_4) { pm_good_15() } else { halt() }
        }
        const p1 = 1;
        function pm_good_16() {
            match_after_5(list)
        }
        if (p1 == rv_4) { pm_good_16() } else { m_alt_7() }
    }
    sorted(rc_7, list)
}
let sort = id(fn_3);
function fn_4(rc_10, list1, list2) {
    function match_after_6(matched_6) {
        rc_10(matched_6)
    }
    function m_alt_8() {
        const d1 = 1;
        let desc_11 = desc(list1);
        function pm_good_17() {
            const i0 = 0;
            let f0_7 = field(list1, i0);
            const i1 = 1;
            let f1_7 = field(list1, i1);
            let x = id(f0_7);
            let rest = id(f1_7);
            function rc_11(rv_7) {
                const d1 = 1;
                let data_Cons_7 = data(d1, x, rv_7);
                match_after_6(data_Cons_7)
            }
            append(rc_11, rest, list2)
        }
        if (d1 == desc_11) { pm_good_17() } else { halt() }
    }
    const d0 = 0;
    let desc_12 = desc(list1);
    function pm_good_18() {
        match_after_6(list2)
    }
    if (d0 == desc_12) { pm_good_18() } else { m_alt_8() }
}
let append = id(fn_4);
function fn_5(rc_12, list) {
    function match_after_7(matched_7) {
        rc_12(matched_7)
    }
    function m_alt_9() {
        const d1 = 1;
        let desc_13 = desc(list);
        function pm_good_19() {
            const i0 = 0;
            let f0_8 = field(list, i0);
            const i1 = 1;
            let f1_8 = field(list, i1);
            let x = id(f0_8);
            let rest = id(f1_8);
            function rc_13(rv_8) {
                match_after_7(rv_8)
            }
            function rc_14(rv_9) {
                const d0 = 0;
                let data_Nil_3 = data(d0);
                const d1 = 1;
                let data_Cons_8 = data(d1, x, data_Nil_3);
                append(rc_13, rv_9, data_Cons_8)
            }
            reverse(rc_14, rest)
        }
        if (d1 == desc_13) { pm_good_19() } else { halt() }
    }
    const d0 = 0;
    let desc_14 = desc(list);
    function pm_good_20() {
        const d0 = 0;
        let data_Nil_4 = data(d0);
        match_after_7(data_Nil_4)
    }
    if (d0 == desc_14) { pm_good_20() } else { m_alt_9() }
}
let reverse = id(fn_5);
const c4 = 4;
const c5 = 5;
const c1 = 1;
const c2 = 2;
const c3 = 3;
const d0 = 0;
let data_Nil_5 = data(d0);
const d1 = 1;
let data_Cons_13 = data(d1, c3, data_Nil_5);
let data_Cons_12 = data(d1, c2, data_Cons_13);
let data_Cons_11 = data(d1, c1, data_Cons_12);
let data_Cons_10 = data(d1, c5, data_Cons_11);
let data_Cons_9 = data(d1, c4, data_Cons_10);
let list = id(data_Cons_9);
function rc_15(rv_10) {
    halt(rv_10)
}
function rc_16(rv_11) {
    reverse(rc_15, rv_11)
}
sort(rc_16, list)
