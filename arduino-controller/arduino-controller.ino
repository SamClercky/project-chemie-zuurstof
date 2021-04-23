#include<Arduino.h>

constexpr int VALVE_LENGTH = 6;
const int VALVE_PINS[VALVE_LENGTH] = {
	6, 7, 2, 3, 5, 4
};

constexpr int INSTRUCTION_MAX_LENGTH = 30; // 5x6 instructies
char instruction[INSTRUCTION_MAX_LENGTH] = "";
int  instr_char = 0;

void setup() {
	// setup serial communication
	Serial.begin(9600);

	// setup pins
	for (int i = 0;  i < VALVE_LENGTH; i++) {
		pinMode(VALVE_PINS[i], OUTPUT);
		digitalWrite(VALVE_PINS[i], HIGH); // give initial value
	}
}

void execCmd(int status) {
	int pin = Serial.parseInt();
	if (0 <= pin && pin < VALVE_LENGTH)
		digitalWrite(VALVE_PINS[pin], status);
}

void loop() {
	if (Serial.available() > 0) { // new op ready
		char newChar = Serial.read();
		
		if (newChar == 'H') {
			execCmd(HIGH);
		} else if (newChar == 'L') {
			execCmd(LOW);
		}
	}
}
