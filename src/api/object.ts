export class ObjectId {
    _id: string
    _mimeType: MimeType

    constructor(id: string, mimeType: MimeType) {
        if (id[0] === "/") {
            this._id = id.substring(1)
        } else {
            this._id = id
        }
        this._mimeType = mimeType
    }

    static directory(id: string) {
        return new ObjectId(id, new MimeType("directory"))
    }

    toString() {
        return this._id
    }

    get id() {
        return this._id
    }

    get mimeType() {
        return this._mimeType
    }
}

export class MimeType {
    _type: string
    _isFolder: boolean

    constructor(mimeType: string) {
        this._type = mimeType
    }

    toString(): string {
        return this._type
    }

    isFolder(): boolean {
        if (this._isFolder === undefined) {
            this._isFolder = ["inode/directory", "directory"].includes(this._type)
        }
        return this._isFolder
    }
}

export interface GenericObject {
    id: ObjectId
    name: string
}

export const serializeFile = (file: GenericObject) => {
    return {
        id: {
            id: file.id._id,
            mimeType: file.id.mimeType.toString()
        },
        name: file.name
    }
}

export interface FileData extends GenericObject {
    lastModified: string
}

export interface Folder extends GenericObject {}