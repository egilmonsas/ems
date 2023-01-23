export interface CrossSectionResponse {
	width: number;
	height: number;
	diameter: number;
	thickness_wall: number;
	thickness_flange: number;
	thickness_web: number;
	radius1: number;
	area: number;
	A_v: number,
	A_v_y: number,
	A_v_z: number,
	I: number;
	I_y: number;
	I_z: number;
	w_el: number;
	w_pl: number;
	w_el_y: number;
	w_pl_y: number;
	w_el_z: number;
	w_pl_z: number;
}
