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

@export var item_scene: PackedScene
@export var mob_scene: PackedScene

var mob_list: Array

# Called when the node enters the scene tree for the first time.
func _ready():
	gamemaster = get_node("GameMaster")
	gamemaster.initialize_level(64, 64)
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
	
	# 落ちているアイテムの情報を取得
	var item_positions = gamemaster.get_dropped_item_positions()
	for pos in item_positions:
		print("Item pos: ", pos)
		var item_inst = item_scene.instantiate()
		item_inst.transform = item_inst.transform.translated(
			get_node("Map").grid_to_geometry(pos))
		add_child(item_inst)

	# 敵の情報を取得
	var mob_positions = gamemaster.get_mob_positions()
	var mob_ids = gamemaster.get_mob_ids()
	for i in range(len(mob_positions)):
		var pos = mob_positions[i]
		var mob_id = mob_ids[i]
		print("Mob pos: ", pos)
		var mob_inst = mob_scene.instantiate()
		mob_inst.mob_id = mob_id
		mob_inst.current_position_2d = pos
		mob_inst.init_position(
			get_node("Map").grid_to_geometry(pos))
		add_child(mob_inst)
		mob_list.append(mob_inst)

	# カメラを切り替え
	get_node("Player/TPCamera3D").make_current()

# モブのアニメーションを実行
func process_mob_animation():
	var mob_positions = gamemaster.get_mob_positions()
	var mob_ids = gamemaster.get_mob_ids()
	for i in range(len(mob_positions)):
		var pos = mob_positions[i]
		var mob_id = mob_ids[i]
		for mob_inst in mob_list:
			if mob_inst.mob_id == mob_id:
				if mob_inst.current_position_2d != pos:
					mob_inst.set_next_position(
						get_node("Map").grid_to_geometry(pos))
					mob_inst.current_position_2d = pos
				break;

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
		var is_move = false
		var is_action = false
		if Input.is_action_pressed("move_up"):
			# (local_x, local_y) をそのままつかう
			# 俯瞰
			direction.y -= 1
			# FPS
			#direction.x += local_x
			#direction.y += local_y
			player_direction = 0
			is_input = true
			is_move = true
		if Input.is_action_pressed("move_left"):
			# (local_y, -local_x) をつかう
			direction.x -= 1
			#direction.x += local_y
			#direction.y -= local_x
			player_direction = 6
			is_input = true
			is_move = true			
		if Input.is_action_pressed("move_down"):
			# (-local_x, -local_y) をつかう
			direction.y += 1
			#direction.x -= local_x
			#direction.y -= local_y
			player_direction = 4
			is_input = true
			is_move = true			
		if Input.is_action_pressed("move_right"):
			# (-local_y, local_x) をつかう
			direction.x += 1
			#direction.x -= local_y
			#direction.y += local_x
			player_direction = 2
			is_input = true
			is_move = true
		if Input.is_action_pressed("rotate_left"):
			player_direction = (player_direction + 3) % 4
			orientation = 1
			is_input = true
		if Input.is_action_pressed("rotate_right"):
			player_direction = (player_direction + 1) % 4	
			orientation = -1
			is_input = true
		if Input.is_action_pressed("apply_button"):
			gamemaster.player_attack();
			is_input = true
			is_action = true

		if is_input:
			if is_move:
				# 移動を指示された場合	
				# GameMasterに問い合わせて移動可能かどうかを決める。
				# 平行移動
				var next_player_position = player_position + direction
				gamemaster.player_turn(player_direction)
				# mapに目標位置に移動可能かどうか問い合わせる
				if gamemaster.player_move(next_player_position):
					# 移動可能だった場合、gamemaster内部の状態はすでに移動済みである。
					# gamemaster側でターンを消費
					gamemaster.process()
					# godot側playerを内部的に移動させる
					player.set_next_position(gridmap.grid_to_geometry(next_player_position))
					player_position = next_player_position
					# アニメーションを実行させる。
					process_mob_animation()
					
				else:
					print("position ", next_player_position, " is invalid, unable to move.")
			elif is_action:
				# プレイヤーがターンを消費する行動を行う場合
				# 今回はattackのみ
				player.set_action(0)
				gamemaster.process()
				# この結果、倒されたmobがいる場合はそのmobを退場させる。
				var defeated_ids = gamemaster.get_defeated_mob_ids()
				for id in defeated_ids:
					#print("defated mob id: ", id)
					for mob_idx in range(len(mob_list)):
						var mob = mob_list[mob_idx]
						#print("  mob[" , mob_idx, "] id: ", mob.mob_id)
						if mob.mob_id == id:
							mob.queue_free()
							mob_list.remove_at(mob_idx)
							break
				# godot側でアニメーションを実行させる。
				process_mob_animation()
			else:
				# 回転移動の場合、ターンは消費されない
				gamemaster.player_turn(player_direction)
				player.set_next_rotation(orientation)
		
	else:
		pass
		#print("moving action blocked.")
