//! 🗂️ Stable stack moves for selected sibling layers, from back to front.
pub fn stack_moves(order:&[String],ids:&[String],operation:&str)->Option<Vec<(String,usize)>> {
    let selected=ids.iter().collect::<std::collections::BTreeSet<_>>();
    let unique=order.iter().collect::<std::collections::BTreeSet<_>>();
    if unique.len()!=order.len() || ids.iter().any(|id|!unique.contains(id)) || !matches!(operation,"bringForward"|"sendBackward"|"bringToFront"|"sendToBack") {return None;}
    let mut working=order.to_vec();
    let mut moves=Vec::new();
    let step=|working:&mut Vec<String>,moves:&mut Vec<(String,usize)>,index:usize,to:usize| {
        let id=working[index].clone();working.swap(index,to);moves.push((id,to));
    };
    if operation=="bringForward" {
        for index in (0..working.len().saturating_sub(1)).rev() {
            if selected.contains(&working[index]) && !selected.contains(&working[index+1]) {step(&mut working,&mut moves,index,index+1);}
        }
    }else if operation=="sendBackward" {
        for index in 1..working.len() {
            if selected.contains(&working[index]) && !selected.contains(&working[index-1]) {step(&mut working,&mut moves,index,index-1);}
        }
    }else {
        let chosen=order.iter().filter(|id|selected.contains(id)).cloned().collect::<Vec<_>>();
        let others=order.iter().filter(|id|!selected.contains(id)).cloned().collect::<Vec<_>>();
        let desired=if operation=="bringToFront" {others.iter().chain(chosen.iter())}else {chosen.iter().chain(others.iter())}.cloned().collect::<Vec<_>>();
        if order==desired {return Some(moves);}
        let mut indices=(0..order.len()).filter(|index|selected.contains(&order[*index])).collect::<Vec<_>>();
        if operation=="sendToBack" {indices.reverse();}
        for original in indices {
            let index=if operation=="bringToFront" {original-moves.len()}else {original+moves.len()};
            let to=if operation=="bringToFront" {order.len()-1}else {0};
            if index!=to {moves.push((order[original].clone(),to));}
        }
    }
    Some(moves)
}
