Instead of having to define each variant of a base.
You instead define the base, and define variants with just differences to the base variant.

```tsx
export type UseComposerProps = UseComposerReplyProps | UseComposerNewProps;

export interface UseComposerBaseProps {
  initialCc?: string[];
  initialBcc?: string[];
}

export interface UseComposerReplyProps {
  inReply: true;
}

export interface UseComposerNewProps {
  inReply?: false;
  initialTo?: string[];
  initialSubject?: string;
}

export interface UseComposerBaseHandle {
  recipients: {
    to: string[];
    cc: string[];
    bcc: string[];
  };
  subject: string;
}

export interface UseComposerReplyHandle extends UseComposerBaseHandle {
  addRecipient: (type: 'cc' | 'bcc', recipient: string) => void;
  removeRecipient: (type: 'cc' | 'bcc', recipient: string) => void;
  setRecipients: (type: 'to' | 'cc' | 'bcc', recipients: string[]) => void;
}

export default function useComposer(props: UseComposerNewProps): UseComposerHandle;
export default function useComposer(props: UseComposerReplyProps): UseComposerHandle;
export default function useComposer({ inReply = false }: UseComposerProps) {}
```