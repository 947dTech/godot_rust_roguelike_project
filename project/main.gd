extends Node3D

var gamemaster

var dungeon_width: int
var dungeon_height: int
var player_position: Vector2i
var goal_position: Vector2i

var goal_reached: bool

var player_direction: int
# UP: -y = 0
# RIGHT: +x = 2
# DOWN: +y = 4
# LEFT: -x = 6
# y = (player_direction / 2 - 1) % 2
# x = (-player_direction / 2 + 2) % 2

@export var item_scene: PackedScene
@export var mob_scene: PackedScene
@export var goal_scene: PackedScene

var item_list: Array
var mob_list: Array

# UI関連の変数
var message_label
var status_label
var item_label
var command_area
var command_label
var selected_command_label

# コマンド選択UI用の変数
var selected_idx
var command_list: Array

# Called when the node enters the scene tree for the first time.
func _ready():
	goal_reached = false
	gamemaster = get_node("/root/GlobalGameMaster")
	# TODO: マップ初期化の際に現在の階層を考慮したレベルデザインを行う。
	gamemaster.initialize_level(64, 64)
	var gridmap = get_node("Map")
	gridmap.initialize_map(gamemaster)
	#dungeon_width = gridmap.dungeon_width
	#dungeon_height = gridmap.dungeon_height
	#print("dungeon_size: ", dungeon_width, " X ", dungeon_height)
	# プレイヤーの初期スポーン地点は移動可能な場所でなければいけない
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
	var item_ids = gamemaster.get_dropped_item_ids()
	# TODO: item_idを取得
	for i in range(len(item_positions)):
		var pos = item_positions[i]
		var item_id = item_ids[i]
		print("Item ", item_id, " pos: ", pos)
		var item_inst = item_scene.instantiate()
		item_inst.item_id = item_id
		item_inst.transform = item_inst.transform.translated(
			get_node("Map").grid_to_geometry(pos))
		add_child(item_inst)
		item_list.append(item_inst)

	# 敵の情報を取得
	var mob_positions = gamemaster.get_mob_positions()
	var mob_ids = gamemaster.get_mob_ids()
	for i in range(len(mob_positions)):
		var pos = mob_positions[i]
		var mob_id = mob_ids[i]
		print("Mob ", mob_id, " pos: ", pos)
		var mob_inst = mob_scene.instantiate()
		mob_inst.mob_id = mob_id
		mob_inst.current_position_2d = pos
		mob_inst.init_position(
			get_node("Map").grid_to_geometry(pos))
		add_child(mob_inst)
		mob_list.append(mob_inst)

	# ゴールを表示
	goal_position = gamemaster.get_goal_position()
	var goal_inst = goal_scene.instantiate()
	goal_inst.transform = goal_inst.transform.translated(
		get_node("Map").grid_to_geometry(goal_position))
	add_child(goal_inst)

	# カメラを切り替え
	get_node("Player/TPCamera3D").make_current()
	
	# UIに文字を表示
	message_label = get_node("Control/MessageArea/MessageLabel")
	message_label.text = "ゲームスタート"
	status_label = get_node("Control/StatusArea/StatusLabel")
	item_label = get_node("Control/ItemArea/ItemLabel")
	update_status_label()
	update_item_label()
	
	# コマンド選択UIは最初は非表示にしておく。
	command_area = get_node("Control/CommandArea")
	command_label = get_node("Control/CommandArea/CommandLabel")
	selected_command_label = get_node("Control/CommandArea/SelectedCommandLabel")
	command_area.visible = false
	selected_idx = 0

# ステータスを表示
func update_status_label():
	status_label.text = gamemaster.get_player_status()

# 所持アイテム一覧を表示
func update_item_label():
	item_label.text = ""
	for item in gamemaster.get_player_items():
		item_label.text += (item + "\n")

# 現在選択可能なコマンド一覧を作成
func update_command_list():
	command_list.clear()
	command_list.append("アイテムを使う")
	command_list.append("装備変更")
	# 次の階層へは自分がゴールの上にいるときだけ移動できる
	if goal_position == player_position:
		command_list.append("次の階層へ移動")
	command_list.append("ゲーム終了")


# コマンド選択UIを表示
func update_command_label():
	selected_command_label.text = ""
	command_label.text = ""
	
	for i in range(selected_idx):
		selected_command_label.text += "\n"
	selected_command_label.text += ">"

	for command in command_list:
		command_label.text += (command + "\n")

# モブのアニメーションを実行
func process_mob_animation():
	var mob_positions = gamemaster.get_mob_positions()
	var mob_ids = gamemaster.get_mob_ids()
	var mob_directions = gamemaster.get_mob_directions()
	for i in range(len(mob_positions)):
		var pos = mob_positions[i]
		var mob_id = mob_ids[i]
		var mob_dir = mob_directions[i]
		for mob_inst in mob_list:
			if mob_inst.mob_id == mob_id:
				mob_inst.set_next_abs_rotation(mob_dir)	
				if mob_inst.current_position_2d != pos:
					print("mob ", mob_id, " moved from ", mob_inst.current_position_2d, " to ", pos)
					mob_inst.set_next_position(
						get_node("Map").grid_to_geometry(pos))
					mob_inst.current_position_2d = pos
				break;

# アイテムが拾われる可能性がある行動が起きたとき、アイテムを削除するかどうか
func remove_dropped_items():
	var dropped_item_removed_ids = gamemaster.get_dropped_item_removed_ids()
	for id in dropped_item_removed_ids:
		for item_idx in range(len(item_list)):
			var item = item_list[item_idx]
			if item.item_id == id:
				item.queue_free()
				item_list.remove_at(item_idx)
				break;

# アイテムが落とされる可能性がある行動が起きたとき、アイテムを追加するかどうか
func add_dropped_items():
	var dropped_item_added_ids = gamemaster.get_dropped_item_added_ids()
	var dropped_item_ids = gamemaster.get_dropped_item_ids()
	var dropped_item_pos = gamemaster.get_dropped_item_positions()
	for item_id in dropped_item_added_ids:
		for item_idx in range(len(dropped_item_ids)):
			if dropped_item_ids[item_idx] == item_id:
				var pos = dropped_item_pos[item_idx]
				var item_inst = item_scene.instantiate()
				item_inst.item_id = item_id
				item_inst.transform = item_inst.transform.translated(
					get_node("Map").grid_to_geometry(pos))
				add_child(item_inst)
				item_list.append(item_inst)
				break

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	# playerに現在入力を受け付けていいかどうか問い合わせる
	var player = get_node("Player")
	var gridmap = get_node("Map")
	if !player.anim_playing:
		# コマンド選択UIを経由して行動を決定する。
		# キャンセルボタンで表示切替を行う。
		if Input.is_action_just_pressed("cancel_button"):
			command_area.visible = !command_area.visible
			# UIを開いた瞬間にUI用のメッセージを生成する
			if command_area.visible:
				update_command_list()
				update_command_label()
		
		# UIを開いている場合は、コマンド選択モードにする。
		if command_area.visible:
			if Input.is_action_just_pressed("move_down"):
				selected_idx = (selected_idx + 1) % command_list.size()
				update_command_label()
			elif Input.is_action_just_pressed("move_up"):
				selected_idx = (selected_idx - 1 + command_list.size()) % command_list.size()
				update_command_label()
			elif Input.is_action_just_pressed("apply_button"):
				# 選択されているコマンドを実行
				if command_list[selected_idx] == "次の階層へ移動":
					gamemaster.goto_scene("res://main.tscn")

		# ゴールに前回の移動の結果乗った場合は次の階層へ移動するかどうかを問い合わせるUIを出す
		elif goal_reached:
			goal_reached = false
			update_command_list()
			update_command_label()
			command_area.visible = true

		# それ以外の場合は直接移動もしくは攻撃
		else:
			# TODO: 俯瞰視点とFPSを切り替えられるようにする。
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
			if Input.is_action_just_pressed("rotate_left"):
				player_direction = (player_direction + 7) % 8
				orientation = 1
				is_input = true
			if Input.is_action_just_pressed("rotate_right"):
				player_direction = (player_direction + 1) % 8	
				orientation = -1
				is_input = true
			if Input.is_action_just_pressed("apply_button"):
				gamemaster.player_attack();
				is_input = true
				is_action = true

			if is_input:
				if is_move:
					# 移動を指示された場合
					gamemaster.clear_message()
					# GameMasterに問い合わせて移動可能かどうかを決める。
					# 平行移動
					var next_player_position = player_position + direction
					gamemaster.player_turn(player_direction)
					player.set_next_abs_rotation(player_direction)
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
						
						# 拾われたアイテムの処理
						remove_dropped_items()
						
						# メッセージの表示
						message_label.text = ""
						for msg_str in gamemaster.message:
							message_label.text += (msg_str + "\n")
						update_status_label()
						update_item_label()

						# 移動した結果ゴールに到達したときはgoal_reachedをtrueに
						if goal_position == player_position:
							goal_reached = true

					else:
						print("position ", next_player_position, " is invalid, unable to move.")
				elif is_action:
					# プレイヤーがターンを消費する行動を行う場合
					gamemaster.clear_message()
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
					
					# 落とされたアイテムの処理
					add_dropped_items()
					
					# メッセージの表示
					message_label.text = ""
					for msg_str in gamemaster.message:
						message_label.text += (msg_str + "\n")
					update_status_label()
					update_item_label()
					
				else:
					# 回転移動の場合、ターンは消費されない
					gamemaster.player_turn(player_direction)
					player.set_next_rotation(orientation)

	else:
		pass
		#print("moving action blocked.")
