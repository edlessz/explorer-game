export interface Vector2 {
	x: number;
	y: number;
}
export interface Vector3 {
	x: number;
	y: number;
	z: number;
}
export interface Transform {
	position: Vector3;
	rotation: number;
	scale: Vector2;
}
export interface Scene {
	entities: SceneEntity[];
}
interface SceneEntity {
	tag: string;
	components: Partial<Record<string, object>>;
}
