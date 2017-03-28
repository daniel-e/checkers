// $(".select_by_class")
// $("#select_by_id")

// TODO: remove my_color from everywhere

var Color = {
  EMPTY: "Empty",
  WHITE_NORMAL: "WhiteNormal",
  WHITE_DAME: "WhiteDame",
  BLACK_NORMAL: "BlackNormal",
  BLACK_DAME: "BlackDame",
};

var PlayerType = {
  HUMAN: "human",
  AI: "ai",
};

var REFRESH_TIMEOUT = 250;
var uid = "";
var last_move_no = -1;
var player_white = PlayerType.HUMAN;
var player_black = PlayerType.AI;


main();

function show_spinning() {
  $(".spinning").show();
}

function hide_spinning() {
  $(".spinning").hide();
}

function hide_status() {
  $(".statusi").html("&nbsp;");
}

function status(msg) {
  $(".statusi").html(msg);
}

var animations = [];

function do_animation(data, idx, f) {
  if (idx >= animations.length) {
    clear_board();
    draw_board(data.board);
    f();
  } else {
    a = animations[idx];
    var i = "piece_" + animations[0][0] + "_" + animations[0][1];
    $("#" + i).animate(
      {'top': get_screeny_for_y(a[3]) + 'px', 'left': get_screenx_for_x(a[2]) + 'px'}, 300, function() {
        do_animation(data, idx + 1, f);
      }
    );
  }
}

function waiting_for_ai() {
  show_spinning();
  status("AI is thinking ...");
}

function update_player(data) {
  var color_of_next_move = data.next_move.toUpperCase();
  if (color_of_next_move == "WHITE") {
    if (player_white == PlayerType.AI) {
      waiting_for_ai();
    } else {
      if (player_black != PlayerType.AI) {
        status("It's white's turn.");
      } else {
        status("It's your turn.");
      }
    }
  } else if (color_of_next_move == "BLACK") {
    if (player_black == PlayerType.AI) {
      waiting_for_ai();
    } else {
      if (player_white != PlayerType.AI) {
        status("It's black's turn.");
      } else {
        status("It's your turn.");
      }
    }
  }
}

function refresh() {
  $.get("/rest/get/" + uid, function(data) {
    if (data.move_no && data.move_no != last_move_no) {  // if something changed
      hide_spinning();
      hide_status();
      last_move_no = data.move_no;
      animations = data.last_moves;
      do_animation(data, 0, function () {
        if (data.winner != "None") {  // if a player has won display the message
          winner(data.winner);
        } else {                      // otherwise continue with the game
          update_player(data);
          setTimeout(refresh, REFRESH_TIMEOUT);
        }
      });
    } else {
      setTimeout(refresh, REFRESH_TIMEOUT);
    }
  }).fail(error);
}

function clear_all() {
  $(".selected").removeClass("selected");
  $(".possible").remove();
}

function clear_board() {
  $(".white").remove();
  $(".black").remove();
  $(".dame").remove();
}

function winner(p) {
  $("#winnertxt").html(p + " wins!");
  $("#winner").show();
}

function move_piece(i, x, y, dx, dy) {

  $("#" + i).animate(
    {'top': get_screeny_for_y(dy) + 'px', 'left': get_screenx_for_x(dx) + 'px'}, 200, function() {
      clear_all();
      $.post("/rest/move/" + uid + "/" + x + "/" + y + "/" + dx + "/" + dy, function(data) {
      })
      .fail(error);
    }
  );
}

function select_piece(i, x, y) {
  var p = $("#boxboard").offset();
  clear_all();
  $.get("/rest/select/" + uid + "/" + x + "/" + y, function(data) {
    if (data.valid_moves.length > 0) {
      $("#" + i).addClass("selected");
      data.valid_moves.forEach(function(e) {
        var dx = e[0];
        var dy = e[1];
        $("<div></div>")
          .css({position: "absolute", left: p.left + 46 + dx * 63.4, top: p.top + 51 + (7 - dy) * 63.2})
          .addClass("possible")
          .appendTo("body")
          .attr("onclick", "move_piece('" + i + "'," + x + "," + y + "," + dx + "," + dy + ")");
      });
    }
  })
  .fail(error);
}

function get_screenx_for_x(x) {
  var p = $("#boxboard").offset();
  return p.left + 52 + x * 63.4;
}

function get_screeny_for_y(y) {
  var p = $("#boxboard").offset();
  return p.top + 56 + (7 - y) * 63.2;
}

function map_color(color) {
  if (color == Color.BLACK_NORMAL || color == Color.BLACK_DAME) {
    return "black";
  } else {
    return "white";
  }
}

// Each piece has an id in the form "piece_<x>_<y>".
function add_piece(x, y, color) {
  if (color != Color.EMPTY) {
    var i = "piece_" + x + "_" + y;
    var p = $("<div></div>")
      .css({position: "absolute", left: get_screenx_for_x(x), top: get_screeny_for_y(y)})
      .attr("id", i)
      .addClass(map_color(color))
      .attr("onclick", "select_piece('" + i + "'," + x + "," + y + ")")
      .appendTo("body");

    if (color == Color.BLACK_DAME || color == Color.WHITE_DAME) {
      p.append($("<div class='piece'><div class='dame'>D</div></div>"));
    }
  }
}

function draw_board(board) {
  for (var i = 0; i < board.length; ++i) {
    add_piece(Math.floor(i % 8), Math.floor(i / 8), board[i]);
  }
}

function error() {
  $("#error").css('z-index', 9999).show();
}

function mode(color, player) {
  $("#" + color + "_human").removeClass("on");
  $("#" + color + "_ai").removeClass("on");
  $("#" + color + "_" + player).addClass("on");
}

function start_game() {
  player_white = $("#white_human").hasClass("on") ? PlayerType.HUMAN : PlayerType.AI;
  player_black = $("#black_human").hasClass("on") ? PlayerType.HUMAN : PlayerType.AI;
  $(".ask").hide();
  $.post("/rest/new/" + player_white + "/" + player_black, function(data) {
    uid = data.uid;
    last_move_no = data.move_no;
    player_black = data.player_black;
    player_white = data.player_white;
    update_player(data);
    draw_board(data.board);
    refresh();
  })
  .fail(error);
}

function main() {
  $("#error").hide();
  $("#winner").hide();
  hide_spinning();
  hide_status();
}
