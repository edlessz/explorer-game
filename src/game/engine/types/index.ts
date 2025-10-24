export interface Vector2 {
	x: number;
	y: number;
}
export interface Transform {
	position: Vector2;
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
