import Component from "../Component";

interface TileRegistryEntry {
	tileId: number;
	name: string;
	assetPath: string;
	solid?: boolean;
	lightIntensity?: number;
	lightRadius?: number;
}

class TileRegistry extends Component {
	private registry: Map<number, TileRegistryEntry> = new Map();

	public registerTiles(entries: TileRegistryEntry[]): void {
		for (const entry of entries) {
			this.registry.set(entry.tileId, entry);
		}
	}
	public registerTile(entry: TileRegistryEntry): void {
		this.registry.set(entry.tileId, entry);
	}
	public getTileEntry(tileId: number): TileRegistryEntry | null {
		return this.registry.get(tileId) ?? null;
	}
}

export default TileRegistry;
