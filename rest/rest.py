#!/usr/bin/env python3

import argparse, uuid, json, threading, multiprocessing, time, collections
from flask import Flask, jsonify

import engine

parser = argparse.ArgumentParser(description = "")
parser.add_argument("--port", type = int, default = 5002, help = "port")
parser.add_argument("--load", type = str, help = "load board setting from file")
parser.add_argument("--depth", type = int, default = 5, help = "maximum search depth")
args = parser.parse_args()

app = Flask(__name__)

q = multiprocessing.Queue()
boards = {}
colors = {}
next_uid = None
board_queues = {}

ai_minimax = lambda q, uid: q.put((uid, engine.ai_minimax(boards[uid], args.depth)))

def update_board(uid, b, player):
    boards[uid] = b
    print("*******", player, json.dumps(
        { "uid": uid, "player_white": colors[uid][0], "player_black": colors[uid][1], "board": b }
    ))
    return json.loads(b)

def recv():
    while True:
        uid, b = q.get()
        update_board(uid, b, "AI")
        queue_add(uid, boards[uid])
        start_ai(uid)

def queue_add(uid, board):
    board_queues[uid].append(board)

def start_ai(uid):
    b = json.loads(boards[uid])
    if b["winner"] != "None":
        return
    if b["next_move"].upper() == "WHITE" and colors[uid][0] == "ai":
        multiprocessing.Process(target = ai_minimax, args = (q, uid)).start()
    if b["next_move"].upper() == "BLACK" and colors[uid][1] == "ai":
        multiprocessing.Process(target = ai_minimax, args = (q, uid)).start()
    # TODO: duplicated code

def nn_game(player_white, player_black):
    global next_uid
    b = engine.new_game()
    if next_uid != None: # use configuration from file
        uid = next_uid
        next_uid = None
        b = boards[uid]
    else:
        uid = str(uuid.uuid4())
    colors[uid] = [player_white, player_black]
    data = update_board(uid, b, "NEW")
    data["player_white"] = player_white
    data["player_black"] = player_black
    data["uid"] = uid
    board_queues[uid] = collections.deque()
    return (uid, data)

# TODO: now receives /human|ai/human|ai
@app.route("/rest/new/<string:player_white>/<string:player_black>", methods = ["POST"])
def new_game(player_white, player_black):
    uid, data = nn_game(player_white, player_black)
    start_ai(uid)
    return jsonify(data)

@app.route("/rest/get/<string:uid>", methods = ["GET"])
def get(uid):
    if uid in board_queues and len(board_queues[uid]) > 0:
        return jsonify(json.loads(board_queues[uid].popleft()))
    else:
        return jsonify({})

@app.route("/rest/select/<string:uid>/<int:x>/<int:y>", methods = ["GET"])
def select(uid, x, y):
    if uid in boards:
        r = { "valid_moves": engine.moves_for(boards[uid], x, y) }
        return jsonify(r)

@app.route("/rest/move/<string:uid>/<int:x>/<int:y>/<int:dx>/<int:dy>", methods = ["POST"])
def move(uid, x, y, dx, dy):
    if uid in boards:
        data = update_board(uid, engine.move_it(boards[uid], x, y, dx, dy), "HUMAN")
        queue_add(uid, boards[uid])
        start_ai(uid)
        return jsonify(data)

if __name__ == "__main__":
    if args.load != None:
        print("Starting with given configuration.")
        data = json.loads(open(args.load).read())
        uid = data["uid"]
        colors[uid] = [data["player_white"], data["player_black"]]
        update_board(uid, data["board"], "LOADED")
        next_uid = uid

    threading.Thread(target = recv).start()
    app.run(port = args.port)
