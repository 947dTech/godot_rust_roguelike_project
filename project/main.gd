extends Node3D

var dungeon_width: int
var dungeon_height: int
var player_position: Vector2i

var player_direction: int
# UP: -y = 0
# DOWN: +y = 2
# RIGHT: +x = 1
# LEFT: -x = 3
# y = (player_direction - 1) % 2
# x = (-player_direction + 2) % 2

# Called when the node enters the scene tree for the first time.
func _ready():
	dungeon_width = get_node("Map").dungeon_width
	dungeon_height = get_node("Map").dungeon_height
	print("dungeon_size: ", dungeon_width, " X ", dungeon_height)
	player_position = Vector2(dungeon_width / 2, dungeon_height / 2)
	get_node("Player").init_position(get_node("Map").grid_to_geometry(player_position))
	player_direction = 0

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	# playerに現在入力を受け付けていいかどうか問い合わせる
	var player = get_node("Player")
	var gridmap = get_node("Map")
	if !player.anim_playing:
		# TODO: 俯瞰視点とFPSを切り替えられるようにする。
		var direction = Vector2i.ZERO
		var orientation = 0
		
		# FPS用の自機方向に対する移動方向の回転
		var local_y = (player_direction - 1) % 2
		var local_x = (-player_direction + 2) % 2
		
		if Input.is_action_pressed("move_up"):
			# (local_x, local_y) をそのままつかう
			# 俯瞰
			#direction.y -= 1
			# FPS
			direction.x += local_x
			direction.y += local_y
		if Input.is_action_pressed("move_left"):
			# (local_y, -local_x) をつかう
			#direction.x -= 1
			direction.x += local_y
			direction.y -= local_x
		if Input.is_action_pressed("move_down"):
			# (-local_x, -local_y) をつかう
			#direction.y += 1
			direction.x -= local_x
			direction.y -= local_y
		if Input.is_action_pressed("move_right"):
			# (-local_y, local_x) をつかう			
			#direction.x += 1
			direction.x -= local_y
			direction.y += local_x
		if Input.is_action_pressed("rotate_left"):
			player_direction = (player_direction + 3) % 4
			orientation = 1
		if Input.is_action_pressed("rotate_right"):
			player_direction = (player_direction + 1) % 4	
			orientation = -1

		# 平行移動
		var next_player_position = player_position + direction
		# mapに目標位置に移動可能かどうか問い合わせる
		if gridmap.map_value(next_player_position.x, next_player_position.y) == 1:
			print("position ", next_player_position, " is invalid, unable to move.")
		else:
			# 移動可能だった場合、playerを内部的に移動させて、アニメーションを実行させる。
			player.set_next_position(gridmap.grid_to_geometry(next_player_position))
			player_position = next_player_position
		
		# 回転移動
		# マップに問い合わせなくてもできるので
		player.set_next_rotation(orientation)
		
	else:
		print("moving action blocked.")
