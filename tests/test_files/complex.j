module complex {
    type Vector3D => {
        x: Number,
        y: Number,
        z: Number
    }

    const ORIGIN = {
        x: 0,
        y: 0,
        z: 0
    }

    let point1 = {
        x: 1,
        y: 0,
        z: 0
    }
    
    let point2 = {
        x: 0,
        y: 1,
        z: 0
    }
    
    let point3 = {
        x: 0,
        y: 0,
        z: 1
    }

    // Testing nested structures
    type Matrix => {
        elements: Vector3D,
        metadata: {
            created: String,
            modified: String
        }
    }
}