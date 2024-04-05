extends Node3D

var gamemaster

var dungeon_width: int
var dungeon_height: int
var player_position: Vector2i

var player_direction: int
# UP: -y = 0
# RIGHT: +x = 2
# DOWN: +y = 4
# LEFT: -x = 6
# y = (player_direction / 2 - 1) % 2
# x = (-player_direction / 2 + 2) % 2

# Called when the node enters the scene tree for the first time.
func _ready():
	gamemaster = get_node("GameMaster")
	gamemaster.initialize_level(100, 100)
	var gridmap = get_node("Map")
	gridmap.initialize_map(gamemaster)
	#dungeon_width = gridmap.dungeon_width
	#dungeon_height = gridmap.dungeon_height
	#print("dungeon_size: ", dungeon_width, " X ", dungeon_height)
	# TODO: プレイヤーの初期スポーン地点は移動可能な場所でなければいけない
	#  そのため、mapmanagerで候補をあげてほしい
	#player_position = Vector2(dungeon_width / 2, dungeon_height / 2)
	player_position = gamemaster.get_player_position()
	print("player position: ", player_position)
	get_node("Player").init_position(get_node("Map").grid_to_geometry(player_position))
	player_direction = gamemaster.get_player_direction()
	
	# debug
	#gamemaster.print_player_status()
	#gamemaster.print_player_items()
	#gamemaster.give_health_potion_to_player()
	#gamemaster.player_attack()

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	# playerに現在入力を受け付けていいかどうか問い合わせる
	var player = get_node("Player")
	var gridmap = get_node("Map")
	if !player.anim_playing:
		# TODO: 俯瞰視点とFPSを切り替えられるようにする。
		# TODO: 自機の向きの符号化をしっかり考える。
		var direction = Vector2i.ZERO
		var orientation = 0
		
		# FPS用の自機方向に対する移動方向の回転
		var local_y = (player_direction - 1) % 2
		var local_x = (-player_direction + 2) % 2
		
		var is_input = false
		if Input.is_action_pressed("move_up"):
			# (local_x, local_y) をそのままつかう
			# 俯瞰
			#direction.y -= 1
			# FPS
			direction.x += local_x
			direction.y += local_y
			is_input = true
		if Input.is_action_pressed("move_left"):
			# (local_y, -local_x) をつかう
			#direction.x -= 1
			direction.x += local_y
			direction.y -= local_x
			is_input = true
		if Input.is_action_pressed("move_down"):
			# (-local_x, -local_y) をつかう
			#direction.y += 1
			direction.x -= local_x
			direction.y -= local_y
			is_input = true
		if Input.is_action_pressed("move_right"):
			# (-local_y, local_x) をつかう
			#direction.x += 1
			direction.x -= local_y
			direction.y += local_x
			is_input = true
		if Input.is_action_pressed("rotate_left"):
			player_direction = (player_direction + 3) % 4
			orientation = 1
			is_input = true
		if Input.is_action_pressed("rotate_right"):
			player_direction = (player_direction + 1) % 4	
			orientation = -1
			is_input = true

		if is_input:
			# GameMasterに問い合わせて移動可能かどうかを決める。
			# 平行移動
			var next_player_position = player_position + direction
			gamemaster.player_turn(player_direction)
			# mapに目標位置に移動可能かどうか問い合わせる
			if gamemaster.player_move(next_player_position):
				# 移動可能だった場合、playerを内部的に移動させて、アニメーションを実行させる。
				player.set_next_position(gridmap.grid_to_geometry(next_player_position))
				player_position = next_player_position
			else:
				print("position ", next_player_position, " is invalid, unable to move.")

			## 回転移動
			#gamemaster.player_turn(player_direction)
			#player.set_next_rotation(orientation)
		
	else:
		pass
		#print("moving action blocked.")
