import type { Vector2 } from "../types";

export type Address = number;

export const decodeAddress = (addr: Address): Vector2 => {
	// Extract x from upper 16 bits (with sign extension)
	const x = addr >> 16;
	// Extract y from lower 16 bits (with sign extension)
	const y = (addr << 16) >> 16;
	return { x, y };
};

export const encodeAddress = (x: number, y: number): Address => {
	// Pack x into upper 16 bits, y into lower 16 bits
	// Supports coordinates from -32768 to 32767
	return (Math.floor(x) << 16) | (Math.floor(y) & 0xffff);
};

export interface MixedColor {
	hex: string;
	intensity: number;
}
export const mixAdditiveColors = (colors: MixedColor[]): MixedColor["hex"] => {
	let r = 0,
		g = 0,
		b = 0;

	for (let { hex, intensity = 1 } of colors) {
		if (hex.startsWith("#")) hex = hex.slice(1);
		if (hex.length !== 6) throw new Error(`Invalid color: ${hex}`);

		// sRGB → linear
		const sr = parseInt(hex.slice(0, 2), 16) / 255;
		const sg = parseInt(hex.slice(2, 4), 16) / 255;
		const sb = parseInt(hex.slice(4, 6), 16) / 255;

		const toLinear = (v: number) =>
			v <= 0.04045 ? v / 12.92 : ((v + 0.055) / 1.055) ** 2.4;

		r += toLinear(sr) * intensity;
		g += toLinear(sg) * intensity;
		b += toLinear(sb) * intensity;
	}

	// Clamp to [0,1]
	r = Math.min(r, 1);
	g = Math.min(g, 1);
	b = Math.min(b, 1);

	// linear → sRGB
	const toSRGB = (v: number) =>
		v <= 0.0031308 ? v * 12.92 : 1.055 * v ** (1 / 2.4) - 0.055;

	const toHex = (v: number) =>
		Math.round(toSRGB(v) * 255)
			.toString(16)
			.padStart(2, "0");

	return `#${toHex(r)}${toHex(g)}${toHex(b)}`;
};
