use crate::prelude::*;

cfg_if! {
    if #[cfg(musl_v1_2)] {
        s! {
            struct __c_anon_sched_param__reserved2 {
                __reserved1: crate::time_t,
                __reserved2: c_long,
            }

            pub struct sched_param {
                pub sched_priority: c_int,

                __reserved1: Padding<c_int>,
                #[cfg(all(
                    musl32_time64,
                    not(all(target_arch = "aarch64", target_pointer_width = "32"))
                ))]
                __reserved2: Padding<[c_long; 4]>,
                #[cfg(any(
                    not(musl32_time64),
                    all(target_arch = "aarch64", target_pointer_width = "32")
                ))]
                __reserved2: Padding<[__c_anon_sched_param__reserved2; 2]>,
                __reserved3: Padding<c_int>,
            }
        }
    } else {
        s! {
            pub struct sched_param {
                pub sched_priority: c_int,

                #[deprecated(since = "0.2.173", note = "This field has been removed upstream")]
                pub sched_ss_low_priority: c_int,
                #[deprecated(since = "0.2.173", note = "This field has been removed upstream")]
                pub sched_ss_repl_period: crate::timespec,
                #[deprecated(since = "0.2.173", note = "This field has been removed upstream")]
                pub sched_ss_init_budget: crate::timespec,
                #[deprecated(since = "0.2.173", note = "This field has been removed upstream")]
                pub sched_ss_max_repl: c_int,
            }
        }
    }
}
