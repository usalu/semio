// #region 🧲️Header
// 💻️ framework/ui/elements/🫀️core/🐹️ElementProps/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header


// #region 🔌️Adapters
import * as React from "react";
import { reactHostPort } from "../🔌Ports/🟦️component.tsx";
// #endregion 🔌️Adapters

// #region 🐹️ElementProps
// Core element types, transaction context, and level-based CSS class helpers.
// Consumers MUST use level functions for consistent styling.

/**
 * Interface for start/finalize/abort lifecycle of a UI transaction.
 **/
export interface Transaction {
  start?: () => void;
  finalize?: () => void;
  abort?: () => void;
}

/**
 * TransactionContext holds the data fields for a TransactionContext record.
 **/
const TransactionContext = reactHostPort.createContext<Transaction | undefined>(undefined);

/**
 * Context provider that supplies a Transaction to descendants.
 **/
export const TransactionProvider: React.FC<{
  transaction?: Transaction;
  children: React.ReactNode;
}> = ({ transaction, children }) => {
  return <TransactionContext.Provider value={transaction}>{children}</TransactionContext.Provider>;
};

/**
 * Hook returning the current Transaction context.
 **/
export const useTransaction = (): Transaction | undefined => reactHostPort.useContext(TransactionContext);

/**
 * Base props interface requiring an id string.
 **/
export interface ElementBaseProps {
  id: string;
}

export interface ElementProps extends ElementBaseProps {}
// #endregion 🐹️ElementProps
