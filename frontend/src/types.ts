export interface UserProfile {
  displayName: string;
  fullName: string;
  email: string;
  avatarUrl: string | null;
}

export interface Kudo {
  id: number;
  senderEmail: string;
  recipientEmail: string;
  message: string;
  createdAt: string;
  isPublic: boolean;
}
