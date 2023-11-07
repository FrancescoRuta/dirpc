#[macro_export]
macro_rules! for_all_tuples {
    ($macro:tt) => {
        $macro!(T0 0;);
        $macro!(T0 0; T1 1;);
        $macro!(T0 0; T1 1; T2 2;);
        $macro!(T0 0; T1 1; T2 2; T3 3;);
        $macro!(T0 0; T1 1; T2 2; T3 3; T4 4;);
        $macro!(T0 0; T1 1; T2 2; T3 3; T4 4; T5 5;);
        $macro!(T0 0; T1 1; T2 2; T3 3; T4 4; T5 5; T6 6;);
        $macro!(T0 0; T1 1; T2 2; T3 3; T4 4; T5 5; T6 6; T7 7;);
        $macro!(T0 0; T1 1; T2 2; T3 3; T4 4; T5 5; T6 6; T7 7; T8 8;);
        $macro!(T0 0; T1 1; T2 2; T3 3; T4 4; T5 5; T6 6; T7 7; T8 8; T9 9;);
        $macro!(T0 0; T1 1; T2 2; T3 3; T4 4; T5 5; T6 6; T7 7; T8 8; T9 9; T10 10;);
        $macro!(T0 0; T1 1; T2 2; T3 3; T4 4; T5 5; T6 6; T7 7; T8 8; T9 9; T10 10; T11 11;);
        $macro!(T0 0; T1 1; T2 2; T3 3; T4 4; T5 5; T6 6; T7 7; T8 8; T9 9; T10 10; T11 11; T12 12;);
        $macro!(T0 0; T1 1; T2 2; T3 3; T4 4; T5 5; T6 6; T7 7; T8 8; T9 9; T10 10; T11 11; T12 12; T13 13;);
        $macro!(T0 0; T1 1; T2 2; T3 3; T4 4; T5 5; T6 6; T7 7; T8 8; T9 9; T10 10; T11 11; T12 12; T13 13; T14 14;);
        $macro!(T0 0; T1 1; T2 2; T3 3; T4 4; T5 5; T6 6; T7 7; T8 8; T9 9; T10 10; T11 11; T12 12; T13 13; T14 14; T15 15;);
        $macro!(T0 0; T1 1; T2 2; T3 3; T4 4; T5 5; T6 6; T7 7; T8 8; T9 9; T10 10; T11 11; T12 12; T13 13; T14 14; T15 15; T16 16;);
    };
}

#[macro_export]
macro_rules! for_all_functions {
    ($macro:tt) => {
        $macro!(T0 t0;);
        $macro!(T0 t0; T1 t1;);
        $macro!(T0 t0; T1 t1; T2 t2;);
        $macro!(T0 t0; T1 t1; T2 t2; T3 t3;);
        $macro!(T0 t0; T1 t1; T2 t2; T3 t3; T4 t4;);
        $macro!(T0 t0; T1 t1; T2 t2; T3 t3; T4 t4; T5 t5;);
        $macro!(T0 t0; T1 t1; T2 t2; T3 t3; T4 t4; T5 t5; T6 t6;);
        $macro!(T0 t0; T1 t1; T2 t2; T3 t3; T4 t4; T5 t5; T6 t6; T7 t7;);
        $macro!(T0 t0; T1 t1; T2 t2; T3 t3; T4 t4; T5 t5; T6 t6; T7 t7; T8 t8;);
        $macro!(T0 t0; T1 t1; T2 t2; T3 t3; T4 t4; T5 t5; T6 t6; T7 t7; T8 t8; T9 t9;);
        $macro!(T0 t0; T1 t1; T2 t2; T3 t3; T4 t4; T5 t5; T6 t6; T7 t7; T8 t8; T9 t9; T10 t10;);
        $macro!(T0 t0; T1 t1; T2 t2; T3 t3; T4 t4; T5 t5; T6 t6; T7 t7; T8 t8; T9 t9; T10 t10; T11 t11;);
        $macro!(T0 t0; T1 t1; T2 t2; T3 t3; T4 t4; T5 t5; T6 t6; T7 t7; T8 t8; T9 t9; T10 t10; T11 t11; T12 t12;);
        $macro!(T0 t0; T1 t1; T2 t2; T3 t3; T4 t4; T5 t5; T6 t6; T7 t7; T8 t8; T9 t9; T10 t10; T11 t11; T12 t12; T13 t13;);
        $macro!(T0 t0; T1 t1; T2 t2; T3 t3; T4 t4; T5 t5; T6 t6; T7 t7; T8 t8; T9 t9; T10 t10; T11 t11; T12 t12; T13 t13; T14 t14;);
        $macro!(T0 t0; T1 t1; T2 t2; T3 t3; T4 t4; T5 t5; T6 t6; T7 t7; T8 t8; T9 t9; T10 t10; T11 t11; T12 t12; T13 t13; T14 t14; T15 t15;);
        $macro!(T0 t0; T1 t1; T2 t2; T3 t3; T4 t4; T5 t5; T6 t6; T7 t7; T8 t8; T9 t9; T10 t10; T11 t11; T12 t12; T13 t13; T14 t14; T15 t15; T16 t16;);
    };
}