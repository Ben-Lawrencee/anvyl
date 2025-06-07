import { Column, ChatMessages } from "@components";
import chat_controller from "@controllers/chat_controller";
import AuthContext from "@contexts/AuthContext";
import ChatContext from "@contexts/ChatContext";
import { use, useState, useCallback } from "react";

export interface ChatViewArguments {
  room: string;
}

export default function ChatRoom({ room }: ChatViewArguments) {
  const { user } = use(AuthContext);
  const chat = chat_controller(8080, room, user);
  const messages = chat.get_messages();

  const [selected, setSelected] = useState<string | undefined>(undefined);

  const onMessageClick = useCallback((messageID: string) => setSelected(messageID), [setSelected]);

  return (
    <ChatContext value={chat.context}>
      <Column gap={2}>
        <ChatMessages messages={messages} selected={selected} onMessageClick={onMessageClick} />
      </Column>
    </ChatContext>
  );
}
