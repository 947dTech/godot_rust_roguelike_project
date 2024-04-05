extends Node3D

@export var floor_chip: PackedScene
@export var wall_chip: PackedScene
@export var chip_size: float = 1.0

var x0 = 0.0
var y0 = 0.0

# Called when the node enters the scene tree for the first time.
func _ready():
	pass

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass

func initialize_map(gamemaster):
	#print(gamemaster.dungeon_width)
	#print(gamemaster.dungeon_height)

	var idx = 0
	x0 = -(gamemaster.dungeon_width / 2.0) * chip_size
	y0 = -(gamemaster.dungeon_height / 2.0) * chip_size
	for x in range(gamemaster.dungeon_width):
		for y in range(gamemaster.dungeon_height):
			idx = x + y * gamemaster.dungeon_width
			var pos = Vector3(x0 + x * chip_size, 0, y0 + y * chip_size)
			if gamemaster.dungeon_map_1d[idx] == 0:
				var fc = floor_chip.instantiate()
				fc.transform = fc.transform.translated(pos)
				add_child(fc)
			elif gamemaster.dungeon_map_1d[idx] == 1:
				var wc = wall_chip.instantiate()
				wc.transform = wc.transform.translated(pos)
				add_child(wc)

# 外部からマップ上のグリッド座標->ユークリッド座標への変換を問い合わせる
func grid_to_geometry(pos):
	return Vector3(
		x0 + pos.x * chip_size,
		0,
		y0 + pos.y * chip_size)
