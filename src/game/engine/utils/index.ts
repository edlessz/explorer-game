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
