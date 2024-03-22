extends GdStaticMapManager

@export var floor_chip: PackedScene
@export var wall_chip: PackedScene
@export var chip_size: float = 1.0

var x0 = 0.0
var y0 = 0.0

# Called when the node enters the scene tree for the first time.
func _ready():
	#generate_map(10, 10)
	#dungeon_map_1d[36] = 1
	generate_dungeon(100, 100)

	# debug print
	#for y in range(dungeon_height):
		#for x in range(dungeon_width):
			#print(" ", dungeon_map_1d[x + dungeon_width * y])
		#print("")
		
	# 各マップチップのシーンをinstantiateする。
	var idx = 0
	x0 = -(dungeon_width / 2.0) * chip_size
	y0 = -(dungeon_height / 2.0) * chip_size
	for x in range(dungeon_width):
		for y in range(dungeon_height):
			idx = x + y * dungeon_width
			var pos = Vector3(x0 + x * chip_size, 0, y0 + y * chip_size)
			if dungeon_map_1d[idx] == 0:
				var fc = floor_chip.instantiate()
				fc.transform = fc.transform.translated(pos)
				add_child(fc)
			elif dungeon_map_1d[idx] == 1:
				var wc = wall_chip.instantiate()
				wc.transform = wc.transform.translated(pos)
				add_child(wc)

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass

# 外部からマップ上のグリッド座標->ユークリッド座標への変換を問い合わせる
func grid_to_geometry(pos):
	return Vector3(
		x0 + pos.x * chip_size,
		0,
		y0 + pos.y * chip_size)
