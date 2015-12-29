#include <Box2D/Box2D.h>
#include "c_b2RevoluteJoint.h"

extern "C" {

	void b2RevoluteJoint_SetMotorSpeed(b2RevoluteJoint* self, float32 speed) {
	    self->SetMotorSpeed(speed);
	}

} // extern C

