#include <Arduino.h>
#include <LiquidCrystal.h>
#include <avr/wdt.h>

constexpr int VALVE_LENGTH = 6;
const int VALVE_PINS[VALVE_LENGTH] = {
	6, 7, 2, 3, 5, 4
};

constexpr int INSTRUCTION_MAX_LENGTH = 30; // 5x6 instructies
char instruction[INSTRUCTION_MAX_LENGTH] = "";
int  instr_char = 0;
LiquidCrystal lcd (12, 13, 8, 9, 10, 11);

// remember to reinit lcd after power break
bool hasRelayChanged = false;

void setup() {
	// setup serial communication
	Serial.begin(9600);
	lcd.begin(16, 2);

	// setup pins
	for (int i = 0;  i < VALVE_LENGTH; i++) {
		pinMode(VALVE_PINS[i], OUTPUT);
		digitalWrite(VALVE_PINS[i], HIGH); // give initial value
	}

	// enable wdt to self heal
	wdt_enable(WDTO_1S);
}

void execCmd(int status) {
	int pin = Serial.parseInt();
	if (0 <= pin && pin < VALVE_LENGTH)
		digitalWrite(VALVE_PINS[pin], status);
	
	// notify LCD
	hasRelayChanged = true;
}

void writeLcdLine(const char* data, int length) {
	char buf[16];
	memset(buf, ' ', sizeof(buf));
	strncpy(buf, data, length);

	if (hasRelayChanged) {
		hasRelayChanged = false;
		lcd.begin(16, 2);
	}

	lcd.setCursor(0, 0);
	lcd.write(buf);
}

void loop() {
	wdt_reset(); // reset wdt timer

	if (Serial.available() > 0) { // new op ready
		char newChar = Serial.read();
		
		if (newChar == 'H') {
			execCmd(LOW); // invert logic because of inverted wiring
		} else if (newChar == 'L') {
			execCmd(HIGH); // invert logic because of wiring
		} else if (newChar == 'M') {
			char msg[16];
			String raw = Serial.readStringUntil('\n');
			raw.toCharArray(msg, 16);

			writeLcdLine("", 1);
			writeLcdLine(msg, raw.length());
		}
	}
}
