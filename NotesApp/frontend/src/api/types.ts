export type INote = {
  id: string;
  title: string;
  content: string;
  is_published: boolean;
  created_at: string;
  updated_at: string;
};

export type IGenericResponse = {
  status: string;
  message: string;
};

export type IErrorResponse = {
  status: string;
  message?: string;
  detail?: string;
};

export type INoteResponse = {
  status: string;
  data: {
    note: INote;
  };
};

export type INotesResponse = {
  status: string;
  count: number;
  notes: INote[];
};