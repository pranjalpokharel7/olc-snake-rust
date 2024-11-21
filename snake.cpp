#include <chrono>
#include <cstdlib>
#include <ctime>
#include <iostream>
#include <list>
#include <thread>
using namespace std;

// headers part of windows C standard
#include <Windows.h>  // need to call some windows function
                      // to set up the console buffer
#include <strsafe.h>  // find an alternative for gcc

int screen_width = 120;
int screen_height = 30;

void clear_screen(wchar_t screen[]) {
    for (int i = 0; i < screen_width * screen_height; i++) {
        screen[i] = L' ';  // L prefix means wchar_t literal
    }
}

struct snake_segment {
    int x, y;
};
enum Direction { UP,
                 RIGHT,
                 DOWN,
                 LEFT };

int main() {
    srand(time(0));

    // wchar or wide characters can support 65536 values
    // i.e. the UNICODE value range instead of 255 by char
    wchar_t* screen = new wchar_t[screen_width * screen_height];

    // everytime you start a terminal session a console buffer
    // is created to store all the character data and color values
    // in arrays
    HANDLE console = CreateConsoleScreenBuffer(GENERIC_READ | GENERIC_WRITE,
                                               0, NULL, CONSOLE_TEXTMODE_BUFFER, NULL);
    clear_screen(screen);
    SetConsoleActiveScreenBuffer(console);
    DWORD dw_bytes_written = 0;  // DWORD defined in windows.h

    list<snake_segment> snake = {{60, 10}, {61, 10}, {62, 10}, {63, 10}, {64, 10}};
    int food_x = 60, food_y = 15;
    int score_count = 0;
    int snake_direction = 3;  // 0 -> North, 1 -> South, 2 -> East, 3 -> West
    bool game_over = false;
    struct direction_keys {
        bool left = false;
        bool right = false;
        bool up = false;
        bool down = false;
    } key;
    int prev_key;

    while (1) {
        while (!game_over) {
            // timing and input

            auto t1 = chrono::system_clock::now();
            auto frame_delay = 220ms;
            auto delta_time = 120ms;

            frame_delay -= delta_time;

            while ((chrono::system_clock::now() - t1) < frame_delay) {
                // see virtual key codes MS docs, 0x27 for right key, 0x25 for left key
                // get async key state returns unsigned short which is 2 bytes long
                // if a key is pressed, the MSB is set, so 0x8000 masks the first bit only
                // which if 1 != 1, returns true (1xxx xxxx xxxx xxxx)
                key.left = (0x8000 & GetAsyncKeyState((unsigned char)('\x25'))) != 0;
                key.up = (0x8000 & GetAsyncKeyState((unsigned char)('\x26'))) != 0;
                key.right = (0x8000 & GetAsyncKeyState((unsigned char)('\x27'))) != 0;
                key.down = (0x8000 & GetAsyncKeyState((unsigned char)('\x28'))) != 0;

                if (key.right && prev_key != RIGHT && snake_direction != LEFT) snake_direction = RIGHT;
                if (key.left && prev_key != LEFT && snake_direction != RIGHT) snake_direction = LEFT;
                if (key.up && prev_key != UP && snake_direction != DOWN) snake_direction = UP;
                if (key.down && prev_key != DOWN && snake_direction != UP) snake_direction = DOWN;

                // default copy constructor called
                prev_key = snake_direction;
            }

            // game logic

            // update snake direction
            switch (snake_direction) {
                case UP:  // up - north
                    snake.push_front({snake.front().x, snake.front().y - 1});
                    break;
                case RIGHT:  // right - east
                    snake.push_front({snake.front().x + 1, snake.front().y});
                    break;
                case DOWN:  // down - south
                    snake.push_front({snake.front().x, snake.front().y + 1});
                    break;
                case LEFT:
                    snake.push_front({snake.front().x - 1, snake.front().y});
                    break;
                default:
                    snake.push_front({snake.front().x, snake.front().y});
                    break;
            }

            // collision detection against screen
            if (snake.front().x < 0 || snake.front().x >= screen_width ||
                snake.front().y < 3 || snake.front().y >= screen_height - 1) {
                game_over = true;
            }

            // collision detection against food
            if (snake.front().x == food_x && snake.front().y == food_y) {
                score_count++;
                // find an empty area on the screen for food coordinates
                // so that there is no overlap between new food coordinate and snake
                while (screen[food_y * screen_width + food_x] != L' ') {
                    food_x = rand() % screen_width;
                    food_y = (rand() % (screen_height - 3)) + 3;
                }
                for (int i = 0; i < 3; i++)
                    snake.push_back({snake.back().x, snake.back().y});
            }

            // collsion detection against itself, bead = individual part of snake
            for (list<snake_segment>::iterator bead = snake.begin();
                 bead != snake.end(); bead++) {
                if (bead != snake.begin() && bead->x == snake.front().x && bead->y == snake.front().y) {
                    game_over = true;
                }
            }

            // pop the snake's tail
            snake.pop_back();

            // display stuff to the player
            clear_screen(screen);

            // draw borders and score
            for (int i = 0; i < screen_width; i++) {
                screen[i] = L'=';
                screen[2 * screen_width + i] = L'=';
                screen[(screen_height - 1) * screen_width + i] = L'=';
            }

            // draw food
            screen[food_y * screen_width + food_x] = L'$';

            wchar_t score_text[] = L"Snake Game in C++\t\t\t\t\tScore:";
            StringCchPrintf(&screen[screen_width + 5], sizeof(score_text),
                            TEXT("%s %d"), score_text, score_count);

            // draw snake body
            for (auto s : snake) {
                screen[s.y * screen_width + s.x] = game_over ? L'+' : L'O';
            }

            // draw snake head
            screen[snake.front().y * screen_width + snake.front().x] = game_over ? L'X' : L'@';

            if (game_over) {
                wchar_t game_over_text[] = L"Game over! Press Space to continue...";
                StringCchPrintf(&screen[screen_width * 10 + 5], sizeof(game_over_text),
                                TEXT("%s"), game_over_text);
            }

            // to write onto screen, put last character of screen
            // array to '\0' the stream terminating character
            screen[screen_width * screen_height - 1] = '\0';
            WriteConsoleOutputCharacter(console, screen, screen_width * screen_height,
                                        {0, 0}, &dw_bytes_written);
        }

        // game over
        if ((0x8000 & GetAsyncKeyState((unsigned char)('\x20'))) != 0) {
            game_over = false;

            // reset snake to original position
            snake = {{60, 10}, {61, 10}, {62, 10}, {63, 10}, {64, 10}};
            snake_direction = 3;
            score_count = 0;
            prev_key = snake_direction;
        }
    }

    delete[] screen;
    return 0;
}