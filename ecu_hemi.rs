 #![no_std]

#[inline(always)]
pub fn sbfx_hemi_stage4(
    launch_pad: bool, 
    pps: u8, 
    rpm: u16, 
    vss_front: u8, 
    vss_rear: u8
) -> (u32, i32, u8, bool) {
    let m_launch = (launch_pad as u32).wrapping_neg();
    let m_launch_u8 = (launch_pad as u8).wrapping_neg();

    let base_throttle = ((pps as u32) * 900) / 255;
    let throttle_target = (base_throttle & !m_launch) | (900 & m_launch);

    let m_cut = ((rpm > 3800) as u8).wrapping_neg() & m_launch_u8;
    let spark_cut_mask = !m_cut;

    let base_timing = 120 + (((rpm as i32) - 800).max(0) * 200 / 5000).min(200);
    let slip = (vss_rear as i32) - (vss_front as i32);
    let slip_excess = (slip - 5).max(0);
    let traction_retard = (slip_excess * 15).min(150);
    let normal_timing = base_timing - traction_retard;
    let launch_timing = -140;

    let ignition_timing = (normal_timing & !(m_launch as i32)) | (launch_timing & (m_launch as i32));

    let nitrous_solenoid_open = !launch_pad && (pps > 240) && (rpm > 3000) && (vss_front > 40);

    (throttle_target, ignition_timing, spark_cut_mask, nitrous_solenoid_open)
}
